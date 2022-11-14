pub struct Palette {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

impl Palette {
    pub fn new(device: &wgpu::Device)
               -> anyhow::Result<Self> {

        let size = wgpu::Extent3d {
            width: 256,
            height: 1,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(
            &wgpu::TextureDescriptor {
                label: Some("Palette texture"),
                size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D1,
                format: wgpu::TextureFormat::Rgba8Uint,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            }
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(
            &wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Nearest,
                min_filter: wgpu::FilterMode::Nearest,
                mipmap_filter: wgpu::FilterMode::Nearest,
                ..Default::default()
            }
        );

        Ok(Self { texture, view, sampler })
    }

    pub fn update(&mut self, queue: &wgpu::Queue, data: &[u8]) {
        assert!(data.len() > 256 * 3);
        let rgba_palette = Self::rgb_to_rgba(data);
        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &rgba_palette,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(256*4),
                rows_per_image: std::num::NonZeroU32::new(1)
            },
            wgpu::Extent3d {
                width: 256,
                height: 1,
                depth_or_array_layers: 1,
            }
        );
    }

    fn rgb_to_rgba(data: &[u8]) -> [u8; 256*4] {
        let mut rgba = [0u8; 256*4];
        for x in 0..256 {
            rgba[(x * 4)] = data[(x * 3)];
            rgba[(x * 4) + 1] = data[(x * 3) + 1];
            rgba[(x * 4) + 2] = data[(x * 3) + 2];
            rgba[(x * 4) + 3] = 0xff;
        }

        rgba
    }

}