
use wgpu::{Device, Queue, Texture, TextureView, TextureDescriptor, TextureFormat, TextureUsages, Extent3d, TextureDimension};
use image::{DynamicImage, GenericImage, GenericImageView, ImageBuffer, imageops, Pixel, Primitive, Rgba};
use std::collections::HashMap;
use std::path::Path;
use imageproc::filter::gaussian_blur_f32;

pub struct TextureManager {
    device: Device,
    queue: Queue,
    textures: HashMap<String, TextureResource>,
}

pub struct TextureResource {
    texture: Texture,
    view: TextureView,
}

impl TextureManager {
    pub fn new(device: Device, queue: Queue) -> Self {
        Self {
            device,
            queue,
            textures: HashMap::new(),
        }
    }

    pub fn load_texture(&mut self, id: &str, path: &str) {
        let img = image::open(path).unwrap();
        let rgba = img.to_rgba8();
        let dimensions = img.dimensions();

        let size = Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        let texture = self.device.create_texture(&TextureDescriptor {
            label: Some(id),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            view_formats: &[],
        });

        self.queue.write_texture(
            texture.as_image_copy(),
            &rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        self.textures.insert(id.to_string(), TextureResource { texture, view });
    }
    pub fn check_for_texture(&self, id: &str) -> bool {
        self.textures.contains_key(id)
    }

    pub fn get_texture_view(&self, id: &str) -> Option<&TextureView> {
        self.textures.get(id).map(|res| &res.view)
    }

    pub fn get_texture(&self, id: &str) -> Option<&Texture> {
        self.textures.get(id).map(|res| &res.texture)
    }

    pub fn update_texture(&mut self, id: &str, path: &str) {
        if let Some(texture_resource) = self.textures.get_mut(id) {
            let img = image::open(path).unwrap();
            let rgba = img.to_rgba8();
            let dimensions = img.dimensions();

            let size = Extent3d {
                width: dimensions.0,
                height: dimensions.1,
                depth_or_array_layers: 1,
            };

            self.queue.write_texture(
                texture_resource.texture.as_image_copy(),
                &rgba,
                wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(4 * dimensions.0),
                    rows_per_image: Some(dimensions.1),
                },
                size,
            );
        }
    }

    pub fn remove_texture(&mut self, id: &str) {
        self.textures.remove(id);
    }

    pub fn clear(&mut self) {
        self.textures.clear();
    }
}

/// Creates a `DynamicImage` from the given width, height and data.
/// The image is then converted to RGBA format and a Gaussian blur is applied to it.
/// The blurred image is then converted back to a `DynamicImage` and returned.
///
/// # Arguments
///
/// * `width` - A u32 that holds the width of the image.
/// * `height` - A u32 that holds the height of the image.
/// * `data` - A 2D array of u8 tuples that represents the RGBA color data for the image.
///
/// # Returns
///
/// * `DynamicImage` - The created image.
///
/// # Example
///
/// ```
///
/// use common::texture_manager::create_texture_tile;
/// let width = 2;
/// let height = 2;
/// let data = [
///     [255, 0, 0, 255],
///     [0, 255, 0, 255],
///     [0, 0, 255, 255],
///     [255, 255, 0, 255],
/// ];
/// let image = create_texture_tile(width, height, data);
/// ```
pub fn create_texture_tile(width: u32, height: u32, data: [[u8; 4]; 4]) -> DynamicImage {
    let data = data.map(|x| Rgba(x)).to_vec();
    let img = ImageBuffer::from_fn(
        width,
        height,
        |x, y| data[(y * width + x) as usize]
    );

    let mut img = DynamicImage::ImageRgba8(img);

    // Define the center and size of the region to blur
    let center_x = width / 2;
    let center_y = height / 2;
    let region_size = 2; // Change this to the size you want

    // Extract the region from the image
    let sub_image = img.crop_imm(center_x - region_size / 2, center_y - region_size / 2, region_size, region_size);

    // Convert the sub-image to RGBA and apply the Gaussian blur
    let rgba = sub_image.to_rgba8();
    let blurred = gaussian_blur_f32(&rgba, 5.0);

    // Convert the blurred image back to a DynamicImage
    let blurred_img = DynamicImage::ImageRgba8(blurred);

    // Insert the blurred image back into the original image
    imageops::replace(&mut img, &blurred_img, (center_x - region_size / 2).into(), (center_y - region_size / 2).into());

    img
}
/// Concatenate horizontally images with the same pixel type.
fn h_concat<I, P, S>(images: &[I]) -> ImageBuffer<P, Vec<S>>
    where
        I: GenericImageView<Pixel = P>,
        P: Pixel<Subpixel = S> + 'static,
        S: Primitive + 'static,
{
    // The final width is the sum of all images width.
    let img_width_out: u32 = images.iter().map(|im| im.width()).sum();

    // The final height is the maximum height from the input images.
    let img_height_out: u32 = images.iter().map(|im| im.height()).max().unwrap_or(0);

    // Initialize an image buffer with the appropriate size.
    let mut imgbuf = image::ImageBuffer::new(img_width_out, img_height_out);
    let mut accumulated_width = 0;

    // Copy each input image at the correct location in the output image.
    for img in images {
        imgbuf.copy_from(img, accumulated_width, 0).unwrap();
        accumulated_width += img.width();
    }

    imgbuf
}

/// Concatenate vertically images with the same pixel type.
/// The images must have the same width.
fn v_concat<I, P, S>(images: &[I]) -> ImageBuffer<P, Vec<S>>
    where
        I: GenericImageView<Pixel = P>,
        P: Pixel<Subpixel = S> + 'static,
        S: Primitive + 'static,
{
    // The final width is the width of the first image.
    let img_width_out = images.first().map(|im| im.width()).unwrap_or(0);

    // The final height is the sum of all images height.
    let img_height_out: u32 = images.iter().map(|im| im.height()).sum();

    // Initialize an image buffer with the appropriate size.
    let mut imgbuf = image::ImageBuffer::new(img_width_out, img_height_out);
    let mut accumulated_height = 0;

    // Copy each input image at the correct location in the output image.
    for img in images {
        imgbuf.copy_from(img, 0, accumulated_height).unwrap();
        accumulated_height += img.height();
    }

    imgbuf
}

/// Concatenate images horizontally
pub fn concat_image_tiles(images: &[&Path]) -> DynamicImage {
    let images: Vec<DynamicImage> = images
        .iter()
        .map(|path|
            image::open(path)
                .expect("Failed to open image")
        )
        .collect();

    DynamicImage::ImageRgba8( h_concat(images.as_slice()) )
}

/// Concatenate image rows vertically
pub fn concat_image_rows(images: Vec<DynamicImage>) -> DynamicImage {
    DynamicImage::ImageRgba8( v_concat(images.as_slice()) )
}

#[cfg(test)]
mod tests {
    use super::*;
    use wgpu::Instance;
    use image::DynamicImage;

    fn create_test_device_and_queue() -> (Device, Queue) {

        let instance_descriptor = wgpu::InstanceDescriptor::default();

        let instance = Instance::new(instance_descriptor);
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: None,
            force_fallback_adapter: false,
        }))
            .unwrap();

        pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                label: None,
            },
            None,
        ))
            .unwrap()
    }

    fn create_mock_image() -> DynamicImage {
        let width = 2;
        let height = 2;
        let data = [
            [200, 200, 210, 200],
            [250, 250, 252, 200],
            [0, 0, 0, 255],
            [180, 180, 180, 250],
        ];
        create_texture_tile(width, height, data)
    }

    fn remove_mock_image(path: &str) {
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_load_texture() {
        let (device, queue) = create_test_device_and_queue();
        let mut texture_manager = TextureManager::new(device, queue);

        let mock_image = create_mock_image();
        let path = "mock_texture.png";
        mock_image.save(path).unwrap();

        texture_manager.load_texture("test_texture", path);
        assert!(texture_manager.get_texture_view("test_texture").is_some());
        remove_mock_image(path);
    }

    #[test]
    fn test_update_texture() {
        let (device, queue) = create_test_device_and_queue();
        let mut texture_manager = TextureManager::new(device, queue);

        let mock_image = create_mock_image();
        let path = "mock_texture.png";
        mock_image.save(path).unwrap();

        texture_manager.load_texture("test_texture", path);
        assert!(texture_manager.get_texture_view("test_texture").is_some());

        let new_mock_image = create_mock_image();
        let new_path = "new_mock_texture.png";
        new_mock_image.save(new_path).unwrap();

        texture_manager.update_texture("test_texture", new_path);
        // Check if the texture view is still valid after update
        assert!(texture_manager.get_texture_view("test_texture").is_some());
        remove_mock_image(path);
        remove_mock_image(new_path);
    }

}