
use nalgebra_glm::{Vec3, Vec4, Mat3, dot, mat4_to_mat3};
use crate::vertex::Vertex;
use crate::Uniforms;
use crate::fragment::Fragment;
use crate::color::Color;
use std::f32::consts::PI;
use rand::Rng;
use rand::SeedableRng;
use rand::rngs::StdRng;

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
    let position = Vec4::new(
        vertex.position.x,
        vertex.position.y,
        vertex.position.z,
        1.0
    );

    let transformed = uniforms.projection_matrix * uniforms.view_matrix * uniforms.model_matrix * position;

    let w = transformed.w;
    let transformed_position = Vec4::new(
        transformed.x / w,
        transformed.y / w,
        transformed.z / w,
        1.0
    );

    let screen_position = uniforms.viewport_matrix * transformed_position;

    let model_mat3 = mat4_to_mat3(&uniforms.model_matrix);
    let normal_matrix = model_mat3.transpose().try_inverse().unwrap_or(Mat3::identity());

    let transformed_normal = normal_matrix * vertex.normal;

    Vertex {
        position: vertex.position,
        normal: vertex.normal,
        tex_coords: vertex.tex_coords,
        color: vertex.color,
        transformed_position: Vec3::new(screen_position.x, screen_position.y, screen_position.z),
        transformed_normal: transformed_normal
    }
}

pub fn fragment_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    black_and_white(fragment, uniforms)
    // dalmata_shader(fragment, uniforms)
    // cloud_shader(fragment, uniforms)
    // cellular_shader(fragment, uniforms)
    // lava_shader(fragment, uniforms)
}

fn black_and_white(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let seed = uniforms.time as f32 * fragment.vertex_position.y * fragment.vertex_position.x;
  
    let mut rng = StdRng::seed_from_u64(seed.abs() as u64);
  
    let random_number = rng.gen_range(0..=100);
  
    let black_or_white = if random_number < 50 {
      Color::new(0, 0, 0)
    } else {
      Color::new(255, 255, 255)
    };
  
    black_or_white * fragment.intensity
}
  
fn dalmata_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let zoom = 100.0;
    let ox = 0.0;
    let oy = 0.0;
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;
  
    let noise_value = uniforms.noise.get_noise_2d(
      (x + ox) * zoom,
      (y + oy) * zoom,
    );
  
    let spot_threshold = 0.5;
    let spot_color = Color::new(255, 255, 255); // White
    let base_color = Color::new(0, 0, 0); // Black
  
    let noise_color = if noise_value < spot_threshold {
      spot_color
    } else {
      base_color
    };
  
    noise_color * fragment.intensity
}
  
fn cloud_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let zoom = 100.0;  // to move our values 
    let ox = 100.0; // offset x in the noise map
    let oy = 100.0;
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;
    let t = uniforms.time as f32 * 0.5;
  
    let noise_value = uniforms.noise.get_noise_2d(x * zoom + ox + t, y * zoom + oy);
  
    // Define cloud threshold and colors
    let cloud_threshold = 0.5; // Adjust this value to change cloud density
    let cloud_color = Color::new(255, 255, 255); // White for clouds
    let sky_color = Color::new(30, 97, 145); // Sky blue
  
    // Determine if the pixel is part of a cloud or sky
    let noise_color = if noise_value > cloud_threshold {
      cloud_color
    } else {
      sky_color
    };
  
    noise_color * fragment.intensity
}
  
fn cellular_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let zoom = 30.0;  // Zoom factor to adjust the scale of the cell pattern
    let ox = 50.0;    // Offset x in the noise map
    let oy = 50.0;    // Offset y in the noise map
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;
  
    // Use a cellular noise function to create the plant cell pattern
    let cell_noise_value = uniforms.noise.get_noise_2d(x * zoom + ox, y * zoom + oy).abs();
  
    // Define different shades of green for the plant cells
    let cell_color_1 = Color::new(85, 107, 47);   // Dark olive green
    let cell_color_2 = Color::new(124, 252, 0);   // Light green
    let cell_color_3 = Color::new(34, 139, 34);   // Forest green
    let cell_color_4 = Color::new(173, 255, 47);  // Yellow green
  
    // Use the noise value to assign a different color to each cell
    let final_color = if cell_noise_value < 0.15 {
      cell_color_1
    } else if cell_noise_value < 0.7 {
      cell_color_2
    } else if cell_noise_value < 0.75 {
      cell_color_3
    } else {
      cell_color_4
    };
  
    // Adjust intensity to simulate lighting effects (optional)
    final_color * fragment.intensity
}
  
fn lava_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Base colors for the lava effect
    let bright_color = Color::new(255, 240, 0); // Bright orange (lava-like)
    let dark_color = Color::new(130, 20, 0);   // Darker red-orange
  
    // Get fragment position
    let position = Vec3::new(
      fragment.vertex_position.x,
      fragment.vertex_position.y,
      fragment.depth
    );
  
    // Base frequency and amplitude for the pulsating effect
    let base_frequency = 0.2;
    let pulsate_amplitude = 0.5;
    let t = uniforms.time as f32 * 0.01;
  
    // Pulsate on the z-axis to change spot size
    let pulsate = (t * base_frequency).sin() * pulsate_amplitude;
  
    // Apply noise to coordinates with subtle pulsating on z-axis
    let zoom = 1000.0; // Constant zoom factor
    let noise_value1 = uniforms.noise.get_noise_3d(
      position.x * zoom,
      position.y * zoom,
      (position.z + pulsate) * zoom
    );
    let noise_value2 = uniforms.noise.get_noise_3d(
      (position.x + 1000.0) * zoom,
      (position.y + 1000.0) * zoom,
      (position.z + 1000.0 + pulsate) * zoom
    );
    let noise_value = (noise_value1 + noise_value2) * 0.5;  // Averaging noise for smoother transitions
  
    // Use lerp for color blending based on noise value
    let color = dark_color.lerp(&bright_color, noise_value);
  
    color * fragment.intensity
}

pub fn star_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let bright_color = Color::new(255, 240, 0); // Naranja brillante (lava)
  let dark_color = Color::new(130, 20, 0);   // Rojo oscuro

  let position = Vec3::new(
      fragment.vertex_position.x,
      fragment.vertex_position.y,
      fragment.depth,
  );

  let base_frequency = 0.2;
  let pulsate_amplitude = 0.5;
  let t = uniforms.time as f32 * 0.01;

  let pulsate = (t * base_frequency).sin() * pulsate_amplitude;

  let zoom = 1000.0;
  let noise_value = uniforms.noise.get_noise_3d(
      position.x * zoom,
      position.y * zoom,
      (position.z + pulsate) * zoom,
  );

  let color = dark_color.lerp(&bright_color, noise_value);

  // Condición para agregar el color al buffer emisivo si es suficientemente brillante
  if noise_value > 0.7 {
      color // Devuelve el color brillante para el buffer principal
  } else {
      color * 0.5 // Devuelve un color más oscuro para zonas no emisivas
  }
}

pub fn earth_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Configuración de colores y propiedades de las nubes
    let earth_green = Color::new(34, 139, 34);
    let ocean_blue = Color::new(70, 130, 180);
    let cloud_color = Color::new(255, 255, 255);
    let cloud_intensity = 0.3; // Ajusta este valor para cambiar la intensidad de las nubes

    // Ajusta el zoom y desplazamiento
    let zoom = 100.0;
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;
    let t = uniforms.time as f32 * 0.01; // Más lento que el Sol

    // Genera el valor de ruido para tierra/agua
    let surface_noise_value = uniforms.noise.get_noise_2d(x * zoom + t, y * zoom + t);

    // Umbral para dividir tierra y océano
    let threshold = 0.3;
    let mut color = if surface_noise_value > threshold {
        earth_green
    } else {
        ocean_blue
    };

    // Genera el valor de ruido para las nubes
    let cloud_noise_value = uniforms.noise.get_noise_2d((x + t * 0.1) * zoom, (y + t * 0.1) * zoom);

    // Agrega las nubes si el ruido de nubes está por encima de un umbral
    let cloud_threshold = 0.6;
    if cloud_noise_value > cloud_threshold {
        color = color.lerp(&cloud_color, cloud_intensity);
    }

    // Ajusta la intensidad para darle efecto de sombreado (como luz y sombra)
    color * fragment.intensity
}



pub fn moon_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let position = fragment.vertex_position;
  let time = uniforms.time as f32 * 0.001;

  // 1. Capa de superficie rocosa con cráteres
  let rock_noise_value = (uniforms.noise.get_noise_3d(position.x * 2.0, position.y * 2.0, position.z * 2.0) + 1.0) / 2.0;
  let base_rock_color = if rock_noise_value > 0.6 {
      Color::new(105, 105, 105) // Gris oscuro para rocas
  } else {
      Color::new(169, 169, 169) // Gris claro para variación en la superficie
  };

  // 2. Capa de cráteres para una textura más irregular
  let crater_noise_value = (uniforms.noise.get_noise_3d(position.x * 10.0, position.y * 10.0, position.z * 10.0) + 1.0) / 2.0;
  let crater_color = Color::new(60, 60, 60); // Color más oscuro para cráteres
  let surface_color = if crater_noise_value < 0.3 {
      crater_color // Agrega cráteres en áreas aleatorias
  } else {
      base_rock_color // Color de la roca en general
  };

  // 3. Efecto de borde caliente
  let glow_noise_value = (uniforms.noise.get_noise_3d(position.x * 0.5 + time, position.y * 0.5 + time, position.z * 0.5 + time) + 1.0) / 2.0;
  let heat_color = Color::new(255, 69, 0); // Naranja rojizo brillante para el borde caliente
  let edge_threshold = 0.7;

  let meteor_color = if glow_noise_value > edge_threshold {
      heat_color // Borde caliente debido a fricción
  } else {
      surface_color // Textura general de la superficie
  };

  // 4. Iluminación para darle profundidad
  let light_dir = Vec3::new(1.0, 1.0, 1.0).normalize(); // Dirección de la luz
  let normal = fragment.normal.normalize();
  let intensity = normal.dot(&light_dir).max(0.3); // Ajuste de intensidad mínima
  let shaded_color = meteor_color * intensity;

  shaded_color
}

pub fn meteor_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let position = fragment.vertex_position;
  let time = uniforms.time as f32 * 0.001;

  // Capa base de color roca
  let rock_noise_value = (uniforms.noise.get_noise_3d(position.x * 3.0, position.y * 3.0, position.z * 3.0) + 1.0) / 2.0;
  let base_rock_color = if rock_noise_value > 0.5 {
      Color::new(105, 105, 105) // Gris oscuro para rocas
  } else {
      Color::new(169, 169, 169) // Gris claro para variación en la superficie
  };

  // Capa de cráteres con mayor frecuencia
  let crater_noise_value = (uniforms.noise.get_noise_3d(position.x * 15.0, position.y * 15.0, position.z * 15.0) + 1.0) / 2.0;
  let crater_color = Color::new(60, 60, 60); // Color más oscuro para cráteres
  let surface_color = if crater_noise_value < 0.4 {
      crater_color // Agrega cráteres en áreas aleatorias
  } else {
      base_rock_color // Color de la roca en general
  };

  // Efecto de borde caliente en el meteorito
  let glow_noise_value = (uniforms.noise.get_noise_3d(position.x * 0.5 + time, position.y * 0.5 + time, position.z * 0.5 + time) + 1.0) / 2.0;
  let heat_color = Color::new(255, 69, 0); // Naranja rojizo brillante para el borde caliente
  let edge_threshold = 0.8;

  let meteor_color = if glow_noise_value > edge_threshold {
      heat_color // Borde caliente debido a fricción
  } else {
      surface_color // Textura general de la superficie
  };

  // Iluminación para agregar profundidad
  let light_dir = Vec3::new(1.0, 1.0, 1.0).normalize(); // Dirección de la luz
  let normal = fragment.normal.normalize();
  let intensity = normal.dot(&light_dir).max(0.3); // Ajuste de intensidad mínima
  let shaded_color = meteor_color * intensity;

  shaded_color
}


pub fn rocky_planet_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let position = fragment.vertex_position;
  let time = uniforms.time as f32 * 0.001;

  // Capa base de color roca con ruido de baja frecuencia
  let base_noise = (uniforms.noise.get_noise_3d(position.x * 2.0, position.y * 2.0, position.z * 2.0) + 1.0) / 2.0;
  let base_color = if base_noise > 0.5 {
      Color::new(139, 69, 19) // Marrón oscuro para zonas rocosas
  } else {
      Color::new(205, 133, 63) // Marrón claro para variación
  };

  // Capa de sombras y grietas (ruido de alta frecuencia)
  let crack_noise = (uniforms.noise.get_noise_3d(position.x * 10.0, position.y * 10.0, position.z * 10.0) + 1.0) / 2.0;
  let crack_color = Color::new(80, 40, 20); // Color más oscuro para grietas y sombras
  let surface_color = if crack_noise < 0.3 {
      crack_color
  } else {
      base_color
  };

  // Simulación de sombras e iluminación en el planeta rocoso
  let light_dir = Vec3::new(1.0, 0.8, 0.6).normalize();
  let normal = fragment.normal.normalize();
  let intensity = normal.dot(&light_dir).max(0.2); // Ajuste de intensidad mínima

  let shaded_color = surface_color * intensity;

  shaded_color
}

pub fn gas_giant_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let position = fragment.vertex_position;
  let time = uniforms.time as f32 * 0.002;

  // Capa de bandas de gas utilizando un patrón de ruido ondulante
  let band_noise = uniforms.noise.get_noise_2d(position.x * 0.5, position.y * 5.0 + time).abs();
  let band_color = if band_noise < 0.5 {
      Color::new(255, 204, 153) // Color más claro para las bandas
  } else {
      Color::new(204, 153, 102) // Color más oscuro para las bandas
  };

  // Capa de remolinos o turbulencias
  let swirl_noise = uniforms.noise.get_noise_3d(position.x * 2.0, position.y * 2.0, time).abs();
  let swirl_color = if swirl_noise > 0.6 {
      Color::new(255, 255, 204) // Remolinos en color claro
  } else {
      band_color
  };

  // Efecto de sombreado suave
  let light_dir = Vec3::new(1.0, -0.5, 0.3).normalize();
  let normal = fragment.normal.normalize();
  let intensity = normal.dot(&light_dir).max(0.4); // Ajuste de intensidad mínima

  let shaded_color = swirl_color * intensity;

  shaded_color
}

pub fn ringed_planet_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let position = fragment.vertex_position;
  let distance_from_center = (position.x.powi(2) + position.y.powi(2)).sqrt();

  // Define los radios internos y externos de los anillos
  let inner_radius = 1.2;
  let outer_radius = 1.5;

  // Si el fragmento está dentro del rango de los anillos, aplica un color de anillo
  let ring_color = if distance_from_center > inner_radius && distance_from_center < outer_radius {
      let ring_pattern = (distance_from_center * 10.0).sin().abs(); // Patrón de bandas
      if ring_pattern > 0.5 {
          Color::new(200, 200, 200) // Color claro para la banda
      } else {
          Color::new(100, 100, 100) // Color oscuro para la banda
      }
  } else {
      // Color del planeta base en el centro
      Color::new(80, 50, 20) // Color del planeta
  };

  ring_color
}

