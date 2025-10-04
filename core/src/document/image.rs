use crate::error::{FilefireError, Result};
use crate::document::{DocumentFormat, ProcessingStats};
use std::collections::HashMap;
use image::{ImageFormat, DynamicImage, ImageError};
use std::io::Cursor;
use kamadak_exif::{Reader as ExifReader, In, Tag, Value};

/// Enterprise-grade image processor with metadata extraction
pub struct ImageProcessor {
    performance_monitor: PerformanceMonitor,
}

impl ImageProcessor {
    pub fn new() -> Self {
        Self {
            performance_monitor: PerformanceMonitor::new(),
        }
    }
    
    /// Process image document with comprehensive analysis
    pub async fn process_document(&mut self, content: &[u8], format: DocumentFormat) -> Result<ProcessedImageDocument> {
        let start_time = std::time::Instant::now();
        
        // Detect image format from content
        let image_format = self.detect_image_format(content, &format)?;
        
        // Load and decode image
        let image = self.load_image(content, &image_format)?;
        
        // Extract basic image properties
        let properties = self.extract_image_properties(&image);
        
        // Extract EXIF metadata
        let exif_data = self.extract_exif_metadata(content)?;
        
        // Extract color information
        let color_analysis = self.analyze_colors(&image)?;
        
        // Detect image features
        let features = self.detect_image_features(&image)?;
        
        // Calculate file hash
        let file_hash = self.calculate_file_hash(content);
        
        // Calculate processing statistics
        let processing_time = start_time.elapsed();
        let stats = ProcessingStats {
            processing_time_ms: processing_time.as_millis() as u64,
            memory_used_mb: self.performance_monitor.get_memory_usage(),
            pages_processed: 1,
            text_extracted_chars: 0,
            images_extracted: 1,
            annotations_found: 0,
            errors_encountered: 0,
            warnings_generated: 0,
        };
        
        Ok(ProcessedImageDocument {
            format: image_format,
            properties,
            exif_data,
            color_analysis,
            features,
            file_hash,
            thumbnail: self.generate_thumbnail(&image)?,
            stats,
        })
    }
    
    /// Detect image format from content and declared format
    fn detect_image_format(&self, content: &[u8], declared_format: &DocumentFormat) -> Result<ImageFormat> {
        // First try to detect from magic bytes
        let detected = image::guess_format(content);
        
        match detected {
            Ok(format) => Ok(format),
            Err(_) => {
                // Fall back to declared format
                match declared_format {
                    DocumentFormat::Jpeg => Ok(ImageFormat::Jpeg),
                    DocumentFormat::Png => Ok(ImageFormat::Png),
                    DocumentFormat::Gif => Ok(ImageFormat::Gif),
                    DocumentFormat::Bmp => Ok(ImageFormat::Bmp),
                    DocumentFormat::Tiff => Ok(ImageFormat::Tiff),
                    DocumentFormat::Webp => Ok(ImageFormat::WebP),
                    DocumentFormat::Svg => Ok(ImageFormat::from_extension("svg").unwrap_or(ImageFormat::Png)),
                    _ => Err(FilefireError::Image("Unable to detect image format".to_string())),
                }
            }
        }
    }
    
    /// Load and decode image from bytes
    fn load_image(&self, content: &[u8], format: &ImageFormat) -> Result<DynamicImage> {
        let cursor = Cursor::new(content);
        
        match image::load(cursor, *format) {
            Ok(img) => Ok(img),
            Err(ImageError::Unsupported(_)) => {
                // Try loading without format specification
                let cursor = Cursor::new(content);
                match image::load_from_memory(content) {
                    Ok(img) => Ok(img), 
                    Err(e) => Err(FilefireError::Image(format!("Failed to load image: {}", e))),
                }
            }
            Err(e) => Err(FilefireError::Image(format!("Failed to load image: {}", e))),
        }
    }
    
    /// Extract basic image properties
    fn extract_image_properties(&self, image: &DynamicImage) -> ImageProperties {
        let (width, height) = image.dimensions();
        let color_type = match image.color() {
            image::ColorType::L8 => "Grayscale",
            image::ColorType::La8 => "Grayscale with Alpha",
            image::ColorType::Rgb8 => "RGB",
            image::ColorType::Rgba8 => "RGBA",
            image::ColorType::L16 => "Grayscale 16-bit",
            image::ColorType::La16 => "Grayscale with Alpha 16-bit",
            image::ColorType::Rgb16 => "RGB 16-bit",
            image::ColorType::Rgba16 => "RGBA 16-bit",
            image::ColorType::Rgb32F => "RGB 32-bit Float",
            image::ColorType::Rgba32F => "RGBA 32-bit Float",
            _ => "Unknown",
        };
        
        let bits_per_pixel = match image.color() {
            image::ColorType::L8 => 8,
            image::ColorType::La8 => 16,
            image::ColorType::Rgb8 => 24,
            image::ColorType::Rgba8 => 32,
            image::ColorType::L16 => 16,
            image::ColorType::La16 => 32,
            image::ColorType::Rgb16 => 48,
            image::ColorType::Rgba16 => 64,
            image::ColorType::Rgb32F => 96,
            image::ColorType::Rgba32F => 128,
            _ => 0,
        };
        
        ImageProperties {
            width,
            height,
            color_type: color_type.to_string(),
            bits_per_pixel,
            has_transparency: matches!(image.color(), 
                image::ColorType::La8 | image::ColorType::Rgba8 | 
                image::ColorType::La16 | image::ColorType::Rgba16 | 
                image::ColorType::Rgba32F
            ),
            aspect_ratio: width as f64 / height as f64,
        }
    }
    
    /// Extract EXIF metadata from image
    fn extract_exif_metadata(&self, content: &[u8]) -> Result<HashMap<String, String>> {
        let mut metadata = HashMap::new();
        
        let cursor = Cursor::new(content);
        match ExifReader::new().read_from_container(cursor) {
            Ok(exif) => {
                // Extract common EXIF fields
                let fields = [
                    (Tag::Make, "camera_make"),
                    (Tag::Model, "camera_model"),
                    (Tag::DateTime, "date_time"),
                    (Tag::DateTimeOriginal, "date_time_original"),
                    (Tag::ExposureTime, "exposure_time"),
                    (Tag::FNumber, "f_number"),
                    (Tag::ISOSpeedRatings, "iso"),
                    (Tag::FocalLength, "focal_length"),
                    (Tag::Flash, "flash"),
                    (Tag::WhiteBalance, "white_balance"),
                    (Tag::ExposureProgram, "exposure_program"),
                    (Tag::MeteringMode, "metering_mode"),
                    (Tag::Orientation, "orientation"),
                    (Tag::XResolution, "x_resolution"),
                    (Tag::YResolution, "y_resolution"),
                    (Tag::ResolutionUnit, "resolution_unit"),
                    (Tag::Software, "software"),
                    (Tag::Artist, "artist"),
                    (Tag::Copyright, "copyright"),
                    (Tag::ImageDescription, "description"),
                    (Tag::GPSLatitude, "gps_latitude"),
                    (Tag::GPSLongitude, "gps_longitude"),
                    (Tag::GPSAltitude, "gps_altitude"),
                ];
                
                for (tag, key) in fields {
                    if let Some(field) = exif.get_field(tag, In::PRIMARY) {
                        let value = match &field.value {
                            Value::Ascii(vec) => {
                                vec.iter()
                                    .map(|bytes| String::from_utf8_lossy(bytes).to_string())
                                    .collect::<Vec<_>>()
                                    .join(", ")
                            }
                            Value::Byte(vec) => {
                                vec.iter()
                                    .map(|b| b.to_string())
                                    .collect::<Vec<_>>()
                                    .join(", ")
                            }
                            Value::Short(vec) => {
                                vec.iter()
                                    .map(|s| s.to_string())
                                    .collect::<Vec<_>>()
                                    .join(", ")
                            }
                            Value::Long(vec) => {
                                vec.iter()
                                    .map(|l| l.to_string())
                                    .collect::<Vec<_>>()
                                    .join(", ")
                            }
                            Value::Rational(vec) => {
                                vec.iter()
                                    .map(|r| format!("{}/{}", r.num, r.denom))
                                    .collect::<Vec<_>>()
                                    .join(", ")
                            }
                            Value::SRational(vec) => {
                                vec.iter()
                                    .map(|r| format!("{}/{}", r.num, r.denom))
                                    .collect::<Vec<_>>()
                                    .join(", ")
                            }
                            Value::Float(vec) => {
                                vec.iter()
                                    .map(|f| f.to_string())
                                    .collect::<Vec<_>>()
                                    .join(", ")
                            }
                            Value::Double(vec) => {
                                vec.iter()
                                    .map(|d| d.to_string())
                                    .collect::<Vec<_>>()
                                    .join(", ")
                            }
                            _ => "Unknown".to_string(),
                        };
                        
                        metadata.insert(key.to_string(), value);
                    }
                }
            }
            Err(e) => {
                log::debug!("No EXIF data found or failed to parse: {}", e);
            }
        }
        
        Ok(metadata)
    }
    
    /// Analyze color composition of image
    fn analyze_colors(&self, image: &DynamicImage) -> Result<ColorAnalysis> {
        let rgb_image = image.to_rgb8();
        let (width, height) = rgb_image.dimensions();
        let pixels = rgb_image.as_raw();
        
        let mut color_histogram: HashMap<[u8; 3], u32> = HashMap::new();
        let mut red_sum = 0u64;
        let mut green_sum = 0u64;
        let mut blue_sum = 0u64;
        let total_pixels = (width * height) as u64;
        
        // Process pixels in chunks of 3 (RGB)
        for chunk in pixels.chunks_exact(3) {
            let rgb = [chunk[0], chunk[1], chunk[2]];
            *color_histogram.entry(rgb).or_insert(0) += 1;
            
            red_sum += chunk[0] as u64;
            green_sum += chunk[1] as u64;
            blue_sum += chunk[2] as u64;
        }
        
        // Calculate average colors
        let avg_red = (red_sum / total_pixels) as u8;
        let avg_green = (green_sum / total_pixels) as u8;
        let avg_blue = (blue_sum / total_pixels) as u8;
        
        // Find dominant colors (top 10)
        let mut color_counts: Vec<([u8; 3], u32)> = color_histogram.into_iter().collect();
        color_counts.sort_by(|a, b| b.1.cmp(&a.1));
        
        let dominant_colors = color_counts
            .into_iter()
            .take(10)
            .map(|(rgb, count)| DominantColor {
                rgb,
                percentage: (count as f64 / total_pixels as f64) * 100.0,
            })
            .collect();
        
        // Analyze brightness
        let brightness = (0.299 * avg_red as f64 + 0.587 * avg_green as f64 + 0.114 * avg_blue as f64) / 255.0;
        
        // Determine if image is predominantly grayscale
        let is_grayscale = self.is_grayscale_image(image);
        
        Ok(ColorAnalysis {
            average_rgb: [avg_red, avg_green, avg_blue],
            dominant_colors,
            brightness,
            is_grayscale,
            unique_colors: color_histogram.len(),
        })
    }
    
    /// Check if image is predominantly grayscale
    fn is_grayscale_image(&self, image: &DynamicImage) -> bool {
        // Sample a subset of pixels to check for grayscale
        let rgb_image = image.to_rgb8();
        let (width, height) = rgb_image.dimensions();
        let pixels = rgb_image.as_raw();
        
        let sample_size = std::cmp::min(1000, pixels.len() / 3);
        let mut grayscale_count = 0;
        
        for i in (0..sample_size * 3).step_by(3) {
            if i + 2 < pixels.len() {
                let r = pixels[i];
                let g = pixels[i + 1];
                let b = pixels[i + 2];
                
                // Check if RGB values are similar (within threshold)
                if (r as i16 - g as i16).abs() < 5 && 
                   (g as i16 - b as i16).abs() < 5 && 
                   (r as i16 - b as i16).abs() < 5 {
                    grayscale_count += 1;
                }
            }
        }
        
        (grayscale_count as f64 / sample_size as f64) > 0.95
    }
    
    /// Detect image features and characteristics
    fn detect_image_features(&self, image: &DynamicImage) -> Result<ImageFeatures> {
        let (width, height) = image.dimensions();
        
        // Classify image size
        let size_category = match (width, height) {
            (w, h) if w <= 150 || h <= 150 => "thumbnail",
            (w, h) if w <= 800 && h <= 600 => "small",
            (w, h) if w <= 1920 && h <= 1080 => "medium",
            (w, h) if w <= 3840 && h <= 2160 => "large",
            _ => "very_large",
        };
        
        // Determine orientation
        let orientation = if width > height {
            "landscape"
        } else if height > width {
            "portrait"
        } else {
            "square"
        };
        
        // Calculate aspect ratio category
        let aspect_ratio = width as f64 / height as f64;
        let aspect_category = match aspect_ratio {
            r if (r - 1.0).abs() < 0.1 => "square",
            r if r > 1.5 => "wide",
            r if r < 0.67 => "tall",
            _ => "standard",
        };
        
        // Simple quality assessment based on resolution
        let megapixels = (width * height) as f64 / 1_000_000.0;
        let quality_score = match megapixels {
            mp if mp < 0.1 => 1.0,  // Very low quality
            mp if mp < 0.5 => 2.0,  // Low quality
            mp if mp < 2.0 => 3.0,  // Medium quality
            mp if mp < 8.0 => 4.0,  // High quality
            _ => 5.0,               // Very high quality
        };
        
        Ok(ImageFeatures {
            size_category: size_category.to_string(),
            orientation: orientation.to_string(),
            aspect_category: aspect_category.to_string(),
            megapixels,
            quality_score,
            estimated_file_size: self.estimate_file_size(width, height, image.color()),
        })
    }
    
    /// Estimate file size based on image properties
    fn estimate_file_size(&self, width: u32, height: u32, color_type: image::ColorType) -> u64 {
        let pixels = width as u64 * height as u64;
        let bytes_per_pixel = match color_type {
            image::ColorType::L8 => 1,
            image::ColorType::La8 => 2,
            image::ColorType::Rgb8 => 3,
            image::ColorType::Rgba8 => 4,
            image::ColorType::L16 => 2,
            image::ColorType::La16 => 4,
            image::ColorType::Rgb16 => 6,
            image::ColorType::Rgba16 => 8,
            image::ColorType::Rgb32F => 12,
            image::ColorType::Rgba32F => 16,
            _ => 3, // Default to RGB
        };
        
        pixels * bytes_per_pixel
    }
    
    /// Calculate file hash for duplicate detection
    fn calculate_file_hash(&self, content: &[u8]) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }
    
    /// Generate thumbnail image
    fn generate_thumbnail(&self, image: &DynamicImage) -> Result<Vec<u8>> {
        let thumbnail = image.thumbnail(200, 200);
        
        let mut buffer = Vec::new();
        let cursor = Cursor::new(&mut buffer);
        
        match thumbnail.write_to(cursor, ImageFormat::Jpeg) {
            Ok(_) => Ok(buffer),
            Err(e) => Err(FilefireError::Image(format!("Failed to generate thumbnail: {}", e))),
        }
    }
}

impl Default for ImageProcessor {
    fn default() -> Self {
        Self::new()
    }
}

/// Performance monitoring for image processing
struct PerformanceMonitor {
    start_memory: usize,
}

impl PerformanceMonitor {
    fn new() -> Self {
        Self {
            start_memory: 0,
        }
    }
    
    fn get_memory_usage(&self) -> f64 {
        // Placeholder for memory usage calculation
        0.0
    }
}

/// Processed image document result
#[derive(Debug, Clone)]
pub struct ProcessedImageDocument {
    pub format: ImageFormat,
    pub properties: ImageProperties,
    pub exif_data: HashMap<String, String>,
    pub color_analysis: ColorAnalysis,
    pub features: ImageFeatures,
    pub file_hash: String,
    pub thumbnail: Vec<u8>,
    pub stats: ProcessingStats,
}

/// Basic image properties
#[derive(Debug, Clone)]
pub struct ImageProperties {
    pub width: u32,
    pub height: u32,
    pub color_type: String,
    pub bits_per_pixel: u8,
    pub has_transparency: bool,
    pub aspect_ratio: f64,
}

/// Color analysis results
#[derive(Debug, Clone)]
pub struct ColorAnalysis {
    pub average_rgb: [u8; 3],
    pub dominant_colors: Vec<DominantColor>,
    pub brightness: f64,
    pub is_grayscale: bool,
    pub unique_colors: usize,
}

/// Dominant color information
#[derive(Debug, Clone)]
pub struct DominantColor {
    pub rgb: [u8; 3],
    pub percentage: f64,
}

/// Image feature analysis
#[derive(Debug, Clone)]
pub struct ImageFeatures {
    pub size_category: String,
    pub orientation: String,
    pub aspect_category: String,
    pub megapixels: f64,
    pub quality_score: f64,  // 1.0-5.0 scale
    pub estimated_file_size: u64,
}