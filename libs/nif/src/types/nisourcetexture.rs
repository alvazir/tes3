// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, PartialEq, SmartDefault)]
pub struct NiSourceTexture {
    pub base: NiTexture,
    #[default(Source::Internal(NiLink::null()))]
    pub source: Source,
    pub pixel_layout: PixelLayout,
    pub use_mipmaps: UseMipMaps,
    pub alpha_format: AlphaFormat,
    #[default(true)]
    pub is_static: bool,
}

impl Load for NiSourceTexture {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let source = stream.load()?;
        let pixel_layout = stream.load()?;
        let use_mipmaps = stream.load()?;
        let alpha_format = stream.load()?;
        let is_static = stream.load::<u8>()? != 0;
        Ok(Self {
            base,
            source,
            pixel_layout,
            use_mipmaps,
            alpha_format,
            is_static,
        })
    }
}

impl Save for NiSourceTexture {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save(&self.source)?;
        stream.save(&self.pixel_layout)?;
        stream.save(&self.use_mipmaps)?;
        stream.save(&self.alpha_format)?;
        stream.save_as::<_, u8>(self.is_static)?;
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, SmartDefault)]
pub enum Source {
    #[default]
    External(String),
    Internal(NiLink<NiPixelData>),
}

impl Load for Source {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let has_external = stream.load::<u8>()? != 0;
        if has_external {
            return Ok(Source::External(stream.load()?));
        }
        let has_internal = stream.load::<u8>()? != 0;
        if has_internal {
            return Ok(Source::Internal(stream.load()?));
        }
        Ok(Source::Internal(NiLink::null()))
    }
}

impl Save for Source {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        match self {
            Source::External(file_name) => {
                stream.save(&1u8)?;
                stream.save_string_without_null_terminator(file_name)?;
            }
            Source::Internal(pixel_data) => {
                stream.save(&0u8)?;
                if pixel_data.is_null() {
                    stream.save(&0u8)?;
                } else {
                    stream.save(&1u8)?;
                    stream.save(pixel_data)?;
                }
            }
        }
        Ok(())
    }
}
