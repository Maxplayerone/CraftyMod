use crate::renderer::buffer::Buffer;
use crate::renderer::vertex_array::VertexArray;
use crate::renderer::vertex_array::VertexArrayConfiguration;

use crate::utils::math;

#[derive(Clone)]
struct Vertex{
    position: math::Vec3,
    uv: [f32; 2],
}

impl Vertex{
    pub fn new(position: math::Vec3, uv: [f32; 2]) -> Self{
        Self{
            position,
            uv,
        }
    }
}

const CHUNK_WIDTH: usize = 16;
const CHUNK_DEPTH: usize = 16;
const CHUNK_HEIGHT: usize = 50;

pub struct Chunk {
    vertices: Vec<Vertex>,
    indices: Vec<i32>,
    cube_count: u32,

    vao: VertexArray,
    vbo: Buffer,
    ibo: Buffer,
}

impl Chunk {
    pub unsafe fn new() -> Self {
        //I don't think they need to be vectors
        //(also I'm not sure about the initializing like every single air block is occupied)
        let vertices = vec![Vertex::new(math::Vec3::new(0.0, 0.0, 0.0), [0.0, 0.0]); 24 * CHUNK_DEPTH * CHUNK_HEIGHT * CHUNK_WIDTH];
        let indices = vec![0; 36 * CHUNK_DEPTH * CHUNK_HEIGHT * CHUNK_WIDTH];

        let vertex_buffer = Buffer::new(gl::ARRAY_BUFFER);
        let element_buffer = Buffer::new(gl::ELEMENT_ARRAY_BUFFER);
        let vertex_array = VertexArray::new();

        Self {
            vertices,
            indices,
            cube_count: 0,
            vao: vertex_array,
            vbo: vertex_buffer,
            ibo: element_buffer,
        }
    }

    fn load_cube(&mut self, pos: &math::Vec3) {
        let size = 1.0;
        //because the indexes start at 0 we want to up them by one
        let indicies_for_quad = 36;
        let vertices_for_quad = 24;

        let index_vert = (self.cube_count * vertices_for_quad) as i32;
        let index_ind = (self.cube_count * indicies_for_quad) as i32;

        //back face
        self.vertices[0 + index_vert as usize] = Vertex::new(math::Vec3::new(pos.x, pos.y, pos.z), [0.0, 0.0]); //left-bottom-back
        self.vertices[1 + index_vert as usize] = Vertex::new(math::Vec3::new(pos.x + size, pos.y, pos.z), [1.0, 0.0]); //right-bottom-back
        self.vertices[2 + index_vert as usize] = Vertex::new(math::Vec3::new(pos.x + size, pos.y + size, pos.z), [1.0, 1.0]); //right-top-back
        self.vertices[3 + index_vert as usize] = Vertex::new(math::Vec3::new(pos.x, pos.y + size, pos.z), [0.0, 1.0]); //left-top-back
        //front face
        self.vertices[4 + index_vert as usize] = Vertex::new(math::Vec3::new(pos.x, pos.y, pos.z + size), [0.0, 0.0]); //left-bottom-front
        self.vertices[5 + index_vert as usize] = Vertex::new(math::Vec3::new(pos.x + size, pos.y, pos.z + size), [1.0, 0.0]); //right-bottom-front
        self.vertices[6 + index_vert as usize] = Vertex::new(math::Vec3::new(pos.x + size, pos.y + size, pos.z + size), [1.0, 1.0]); //right-top-front
        self.vertices[7 + index_vert as usize] = Vertex::new(math::Vec3::new(pos.x, pos.y + size, pos.z + size), [0.0, 1.0]); //left-top-front
        //left face
        self.vertices[8 + index_vert as usize] = Vertex::new(math::Vec3::new(pos.x, pos.y + size, pos.z + size), [1.0, 0.0]); //left-top-front
        self.vertices[9 + index_vert as usize] = Vertex::new(math::Vec3::new(pos.x, pos.y + size, pos.z), [1.0, 1.0]); //left-top-back
        self.vertices[10 + index_vert as usize] = Vertex::new(math::Vec3::new(pos.x, pos.y, pos.z), [0.0, 1.0]); //left-bottom-back
        self.vertices[11 + index_vert as usize] = Vertex::new(math::Vec3::new(pos.x, pos.y, pos.z + size), [0.0, 0.0]); //left-bottom-front
        //right face
        self.vertices[12 + index_vert as usize] = Vertex::new(math::Vec3::new(pos.x + size, pos.y + size, pos.z + size), [1.0, 0.0]); //right-top-front
        self.vertices[13 + index_vert as usize] = Vertex::new(math::Vec3::new(pos.x + size, pos.y + size, pos.z), [1.0, 1.0]); //right-top-back
        self.vertices[14 + index_vert as usize] = Vertex::new(math::Vec3::new(pos.x + size, pos.y, pos.z), [0.0, 1.0]); //right-bottom-back
        self.vertices[15 + index_vert as usize] = Vertex::new(math::Vec3::new(pos.x + size, pos.y, pos.z + size), [0.0, 0.0]); //right-bottom-front
        //bottom face
        self.vertices[16 + index_vert as usize] = Vertex::new(math::Vec3::new(pos.x, pos.y, pos.z), [0.0, 1.0]); //left-bottom-back
        self.vertices[17 + index_vert as usize] = Vertex::new(math::Vec3::new(pos.x + size, pos.y, pos.z), [1.0, 1.0]); //right-bottom-back
        self.vertices[18 + index_vert as usize] = Vertex::new(math::Vec3::new(pos.x + size, pos.y, pos.z + size), [1.0, 0.0]); //right-bottom-front
        self.vertices[19 + index_vert as usize] = Vertex::new(math::Vec3::new(pos.x, pos.y, pos.z + size), [0.0, 0.0]); //left-bottom-front
        //bottom face
        self.vertices[20 + index_vert as usize] = Vertex::new(math::Vec3::new(pos.x, pos.y + size, pos.z), [0.0, 1.0]); //left-top-back
        self.vertices[21 + index_vert as usize] = Vertex::new(math::Vec3::new(pos.x + size, pos.y + size, pos.z), [1.0, 1.0]); //right-top-back
        self.vertices[22 + index_vert as usize] = Vertex::new(math::Vec3::new(pos.x + size, pos.y + size, pos.z + size), [1.0, 0.0]); //right-top-front
        self.vertices[23 + index_vert as usize] = Vertex::new(math::Vec3::new(pos.x, pos.y + size, pos.z + size), [0.0, 0.0]); //left-top-front       

        self.indices[0 + index_ind as usize] = 2 + index_vert;
        self.indices[1 + index_ind as usize] = 1 + index_vert;
        self.indices[2 + index_ind as usize] = 0 + index_vert;
        self.indices[3 + index_ind as usize] = 0 + index_vert;
        self.indices[4 + index_ind as usize] = 3 + index_vert;
        self.indices[5 + index_ind as usize] = 2 + index_vert;
        self.indices[6 + index_ind as usize] = 5 + index_vert;
        self.indices[7 + index_ind as usize] = 6 + index_vert;
        self.indices[8 + index_ind as usize] = 7 + index_vert;
        self.indices[9 + index_ind as usize] = 7 + index_vert;
        self.indices[10 + index_ind as usize] = 4 + index_vert;
        self.indices[11 + index_ind as usize] = 5 + index_vert;
        self.indices[12 + index_ind as usize] = 8 + index_vert;
        self.indices[13 + index_ind as usize] = 9 + index_vert;
        self.indices[14 + index_ind as usize] = 10 + index_vert;
        self.indices[15 + index_ind as usize] = 10 + index_vert;
        self.indices[16 + index_ind as usize] = 11 + index_vert;
        self.indices[17 + index_ind as usize] = 8 + index_vert;
        self.indices[18 + index_ind as usize] = 14 + index_vert;
        self.indices[19 + index_ind as usize] = 13 + index_vert;
        self.indices[20 + index_ind as usize] = 12 + index_vert;
        self.indices[21 + index_ind as usize] = 12 + index_vert;
        self.indices[22 + index_ind as usize] = 15 + index_vert;
        self.indices[23 + index_ind as usize] = 14 + index_vert;
        self.indices[24 + index_ind as usize] = 16 + index_vert;
        self.indices[25 + index_ind as usize] = 17 + index_vert;
        self.indices[26 + index_ind as usize] = 18 + index_vert;
        self.indices[27 + index_ind as usize] = 18 + index_vert;
        self.indices[28 + index_ind as usize] = 19 + index_vert;
        self.indices[29 + index_ind as usize] = 16 + index_vert;
        self.indices[30 + index_ind as usize] = 22 + index_vert;
        self.indices[31 + index_ind as usize] = 21 + index_vert;
        self.indices[32 + index_ind as usize] = 20 + index_vert;
        self.indices[33 + index_ind as usize] = 20 + index_vert;
        self.indices[34 + index_ind as usize] = 23 + index_vert;
        self.indices[35 + index_ind as usize] = 22 + index_vert;

        self.cube_count += 1;
    }

    fn vertices(&self) -> Vec<f32>{
        let mut vertices: Vec<f32> =  vec![0.0; self.vertices.len() * 5];
        let mut vertices_marker = 0;
        for vertex in self.vertices.iter(){
            vertices[vertices_marker] = vertex.position.x;
            vertices_marker += 1;
            vertices[vertices_marker] = vertex.position.y;
            vertices_marker += 1;
            vertices[vertices_marker] = vertex.position.z;
            vertices_marker += 1;
            vertices[vertices_marker] = vertex.uv[0];
            vertices_marker += 1;
            vertices[vertices_marker] = vertex.uv[1];
            vertices_marker += 1;
        }

        vertices
    }

    pub unsafe fn load_cubes(&mut self, pos: math::Vec3) {
        //for y in 0..CHUNK_HEIGHT{
        for z in 0..CHUNK_DEPTH{
            for x in 0..CHUNK_WIDTH{
                self.load_cube(&math::Vec3::new(pos.x + x as f32, pos.y, pos.z + z as f32));
            }
        }
    //}

        self.vao.bind();
        self.vbo.set_data(&self.vertices(), gl::STATIC_DRAW);
        self.ibo.set_data(&self.indices, gl::STATIC_DRAW);
        self.vao
            .setup_vao(VertexArrayConfiguration::XyzAndTexCoords);
    }
    /*
        pub fn coordinate_to_block_index(pos: math::Vec3){

        }
    */
    pub unsafe fn render(&mut self) {
        self.vao.bind();
        gl::DrawElements(
            gl::TRIANGLES,
            self.indices.len().try_into().unwrap(),
            gl::UNSIGNED_INT,
            std::ptr::null(),
        );
    }
}
