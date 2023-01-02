use crate::renderer::buffer::Buffer;
use crate::renderer::vertex_array::VertexArray;
use crate::renderer::vertex_array::VertexArrayConfiguration;

use crate::utils::math;

pub struct Block {
    pub vertices: [f32; 120],
    pub indices: [u32; 36],
}

impl Block {
    pub fn new(pos: math::Vec3) -> Self {
        let size = 1.0;
        Self {
            //change the hard-coded values in the second arguement to the vertex new function
            //when you'll be using a texture atlas
            #[rustfmt::skip]
            vertices: [
                //back face
                pos.x, pos.y, pos.z, 0.0, 0.0, //left-bottom-back
                pos.x + size, pos.y, pos.z, 1.0, 0.0, //right-bottom-back
                pos.x + size, pos.y + size, pos.z, 1.0, 1.0, //right-top-back
                pos.x, pos.y + size, pos.z, 0.0, 1.0, //left-top-back
                
                //front face
                pos.x, pos.y, pos.z + size, 0.0, 0.0, //left-bottom-front
                pos.x + size, pos.y, pos.z + size, 1.0, 0.0, //right-bottom-front
                pos.x + size, pos.y + size, pos.z + size, 1.0, 1.0, //right-top-front
                pos.x, pos.y + size, pos.z + size, 0.0, 1.0, //left-top-front
            
                //left face
                pos.x, pos.y + size, pos.z + size, 1.0, 0.0, //left-top-front
                pos.x, pos.y + size, pos.z, 1.0, 1.0, //left-top-back
                pos.x, pos.y, pos.z, 0.0, 1.0, //left-bottom-back
                pos.x, pos.y, pos.z + size, 0.0, 0.0, //left-bottom-front
                
                //right face
                pos.x + size, pos.y + size, pos.z + size, 1.0, 0.0, //right-tom-front
                pos.x + size, pos.y + size, pos.z, 1.0, 1.0, //right-top-back
                pos.x + size, pos.y, pos.z, 0.0, 1.0, //right-bottom-back
                pos.x + size, pos.y, pos.z + size, 0.0, 0.0, //right-bottom-front

                //bottom face
                pos.x, pos.y, pos.z, 0.0, 1.0, //left-bottom-back
                pos.x + size, pos.y, pos.z, 1.0, 1.0, //right-bottom-back
                pos.x + size, pos.y, pos.z + size, 1.0, 0.0, //right-bottom-front
                pos.x, pos.y, pos.z + size, 0.0, 0.0, //left-bottom-front

                //top face
                pos.x, pos.y + size, pos.z, 0.0, 1.0, //left-top-back
                pos.x + size, pos.y + size, pos.z, 1.0, 1.0, //right-top-back
                pos.x + size, pos.y + size, pos.z + size, 1.0, 0.0, //right-top-front
                pos.x, pos.y + size, pos.z + size, 0.0, 0.0, //left-top-front
            ],
            #[rustfmt::skip]
            indices: [2, 1, 0, 0, 3, 2,
                    5, 6, 7, 7, 4, 5,
                    8, 9, 10, 10, 11, 8,
                    14, 13, 12, 12, 15, 14,
                    16, 17, 18, 18, 19, 16,
                    22, 21, 20, 20, 23, 22],
        }
    }
}

pub struct Chunk {
    vertices: Vec<f32>,
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
        let vertices = vec![0.0; 240];
        let indices = vec![0; 72];

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

    fn load_cube(&mut self, pos: &math::Vec3, index: i32) {
        let size = 1.0;
        //because the indexes start at 0 we want to up them by one
        let indicies_for_quad = 36;

        let mut index_cpy = index;
        let index_vert_zero = index_cpy * 120;
        let index_vert = index_cpy * 120;

        let index_ind_zero = index_cpy * indicies_for_quad;
        let index_ind = index_cpy * indicies_for_quad;

        //back face
        self.vertices[0 + index_vert_zero as usize] = pos.x; //left-bottom-back
        self.vertices[1 + index_vert as usize] = pos.y;
        self.vertices[2 + index_vert as usize] = pos.z;
        self.vertices[3 + index_vert as usize] = 0.0;
        self.vertices[4 + index_vert as usize] = 0.0;
        self.vertices[5 + index_vert as usize] = pos.x + size; //right-bottom-back
        self.vertices[6 + index_vert as usize] = pos.y;
        self.vertices[7 + index_vert as usize] = pos.z;
        self.vertices[8 + index_vert as usize] = 1.0;
        self.vertices[9 + index_vert as usize] = 0.0;
        self.vertices[10 + index_vert as usize] = pos.x + size; //right-top-back
        self.vertices[11 + index_vert as usize] = pos.y + size;
        self.vertices[12 + index_vert as usize] = pos.z;
        self.vertices[13 + index_vert as usize] = 1.0;
        self.vertices[14 + index_vert as usize] = 1.0;
        self.vertices[15 + index_vert as usize] = pos.x;
        self.vertices[16 + index_vert as usize] = pos.y + size; //left-top-back
        self.vertices[17 + index_vert as usize] = pos.z;
        self.vertices[18 + index_vert as usize] = 0.0;
        self.vertices[19 + index_vert as usize] = 1.0;

        //front face
        self.vertices[20 + index_vert as usize] = pos.x; //left-bottom-front
        self.vertices[21 + index_vert as usize] = pos.y;
        self.vertices[22 + index_vert as usize] = pos.z + size;
        self.vertices[23 + index_vert as usize] = 0.0;
        self.vertices[24 + index_vert as usize] = 0.0;
        self.vertices[25 + index_vert as usize] = pos.x + size; //right-bottom-front
        self.vertices[26 + index_vert as usize] = pos.y;
        self.vertices[27 + index_vert as usize] = pos.z + size;
        self.vertices[28 + index_vert as usize] = 1.0;
        self.vertices[29 + index_vert as usize] = 0.0;
        self.vertices[30 + index_vert as usize] = pos.x + size; //right-top-front
        self.vertices[31 + index_vert as usize] = pos.y + size;
        self.vertices[32 + index_vert as usize] = pos.z + size;
        self.vertices[33 + index_vert as usize] = 1.0;
        self.vertices[34 + index_vert as usize] = 1.0;
        self.vertices[35 + index_vert as usize] = pos.x;
        self.vertices[36 + index_vert as usize] = pos.y + size; //left-top-front
        self.vertices[37 + index_vert as usize] = pos.z + size;
        self.vertices[38 + index_vert as usize] = 0.0;
        self.vertices[39 + index_vert as usize] = 1.0;

        //left face
        self.vertices[40 + index_vert as usize] = pos.x; //left-top-front
        self.vertices[41 + index_vert as usize] = pos.y + size;
        self.vertices[42 + index_vert as usize] = pos.z + size;
        self.vertices[43 + index_vert as usize] = 1.0;
        self.vertices[44 + index_vert as usize] = 0.0;
        self.vertices[45 + index_vert as usize] = pos.x; //left-top-back
        self.vertices[46 + index_vert as usize] = pos.y + size;
        self.vertices[47 + index_vert as usize] = pos.z;
        self.vertices[48 + index_vert as usize] = 1.0;
        self.vertices[49 + index_vert as usize] = 1.0;
        self.vertices[50 + index_vert as usize] = pos.x; //left-bottom-back
        self.vertices[51 + index_vert as usize] = pos.y;
        self.vertices[52 + index_vert as usize] = pos.z;
        self.vertices[53 + index_vert as usize] = 0.0;
        self.vertices[54 + index_vert as usize] = 1.0;
        self.vertices[55 + index_vert as usize] = pos.x;
        self.vertices[56 + index_vert as usize] = pos.y; //left-bottom-front
        self.vertices[57 + index_vert as usize] = pos.z + size;
        self.vertices[58 + index_vert as usize] = 0.0;
        self.vertices[59 + index_vert as usize] = 0.0;

        //right face
        self.vertices[60 + index_vert as usize] = pos.x + size; //right-top-front
        self.vertices[61 + index_vert as usize] = pos.y + size;
        self.vertices[62 + index_vert as usize] = pos.z + size;
        self.vertices[63 + index_vert as usize] = 1.0;
        self.vertices[64 + index_vert as usize] = 0.0;
        self.vertices[65 + index_vert as usize] = pos.x + size; //right-top-back
        self.vertices[66 + index_vert as usize] = pos.y + size;
        self.vertices[67 + index_vert as usize] = pos.z;
        self.vertices[68 + index_vert as usize] = 1.0;
        self.vertices[69 + index_vert as usize] = 1.0;
        self.vertices[70 + index_vert as usize] = pos.x + size; //right-bottom-back
        self.vertices[71 + index_vert as usize] = pos.y;
        self.vertices[72 + index_vert as usize] = pos.z;
        self.vertices[73 + index_vert as usize] = 0.0;
        self.vertices[74 + index_vert as usize] = 1.0;
        self.vertices[75 + index_vert as usize] = pos.x + size;
        self.vertices[76 + index_vert as usize] = pos.y; //right-bottom-front
        self.vertices[77 + index_vert as usize] = pos.z + size;
        self.vertices[78 + index_vert as usize] = 0.0;
        self.vertices[79 + index_vert as usize] = 0.0;

        //bottom face
        self.vertices[80 + index_vert as usize] = pos.x; //left-bottom-back
        self.vertices[81 + index_vert as usize] = pos.y;
        self.vertices[82 + index_vert as usize] = pos.z;
        self.vertices[83 + index_vert as usize] = 0.0;
        self.vertices[84 + index_vert as usize] = 1.0;
        self.vertices[85 + index_vert as usize] = pos.x + size; //right-bottom-back
        self.vertices[86 + index_vert as usize] = pos.y;
        self.vertices[87 + index_vert as usize] = pos.z;
        self.vertices[88 + index_vert as usize] = 1.0;
        self.vertices[89 + index_vert as usize] = 1.0;
        self.vertices[90 + index_vert as usize] = pos.x + size; //right-bottom-front
        self.vertices[91 + index_vert as usize] = pos.y;
        self.vertices[92 + index_vert as usize] = pos.z + size;
        self.vertices[93 + index_vert as usize] = 1.0;
        self.vertices[94 + index_vert as usize] = 0.0;
        self.vertices[95 + index_vert as usize] = pos.x; //left-bottom-front
        self.vertices[96 + index_vert as usize] = pos.y;
        self.vertices[97 + index_vert as usize] = pos.z + size;
        self.vertices[98 + index_vert as usize] = 0.0;
        self.vertices[99 + index_vert as usize] = 0.0;

        //top face
        self.vertices[100 + index_vert as usize] = pos.x; //left-top-back
        self.vertices[101 + index_vert as usize] = pos.y + size;
        self.vertices[102 + index_vert as usize] = pos.z;
        self.vertices[103 + index_vert as usize] = 0.0;
        self.vertices[104 + index_vert as usize] = 1.0;
        self.vertices[105 + index_vert as usize] = pos.x + size; //right-top-back
        self.vertices[106 + index_vert as usize] = pos.y + size;
        self.vertices[107 + index_vert as usize] = pos.z;
        self.vertices[108 + index_vert as usize] = 1.0;
        self.vertices[109 + index_vert as usize] = 1.0;
        self.vertices[110 + index_vert as usize] = pos.x + size; //right-top-front
        self.vertices[111 + index_vert as usize] = pos.y + size;
        self.vertices[112 + index_vert as usize] = pos.z + size;
        self.vertices[113 + index_vert as usize] = 1.0;
        self.vertices[114 + index_vert as usize] = 0.0;
        self.vertices[115 + index_vert as usize] = pos.x; //left-top-front
        self.vertices[116 + index_vert as usize] = pos.y + size;
        self.vertices[117 + index_vert as usize] = pos.z + size;
        self.vertices[118 + index_vert as usize] = 0.0;
        self.vertices[119 + index_vert as usize] = 0.0;

        self.indices[0 + index_ind_zero as usize] = 2 + indicies_for_quad * index_cpy;
        self.indices[1 + index_ind as usize] = 1 + indicies_for_quad * index_cpy;
        self.indices[2 + index_ind as usize] = 0 + indicies_for_quad * index_cpy;
        self.indices[3 + index_ind as usize] = 0 + indicies_for_quad * index_cpy;
        self.indices[4 + index_ind as usize] = 3 + indicies_for_quad * index_cpy;
        self.indices[5 + index_ind as usize] = 2 + indicies_for_quad * index_cpy;
        self.indices[6 + index_ind as usize] = 5 + indicies_for_quad * index_cpy;
        self.indices[7 + index_ind as usize] = 6 + indicies_for_quad * index_cpy;
        self.indices[8 + index_ind as usize] = 7 + indicies_for_quad * index_cpy;
        self.indices[9 + index_ind as usize] = 7 + indicies_for_quad * index_cpy;
        self.indices[10 + index_ind as usize] = 4 + indicies_for_quad * index_cpy;
        self.indices[11 + index_ind as usize] = 5 + indicies_for_quad * index_cpy;
        self.indices[12 + index_ind as usize] = 8 + indicies_for_quad * index_cpy;
        self.indices[13 + index_ind as usize] = 9 + indicies_for_quad * index_cpy;
        self.indices[14 + index_ind as usize] = 10 + indicies_for_quad * index_cpy;
        self.indices[15 + index_ind as usize] = 10 + indicies_for_quad * index_cpy;
        self.indices[16 + index_ind as usize] = 11 + indicies_for_quad * index_cpy;
        self.indices[17 + index_ind as usize] = 8 + indicies_for_quad * index_cpy;
        self.indices[18 + index_ind as usize] = 14 + indicies_for_quad * index_cpy;
        self.indices[19 + index_ind as usize] = 13 + indicies_for_quad * index_cpy;
        self.indices[20 + index_ind as usize] = 12 + indicies_for_quad * index_cpy;
        self.indices[21 + index_ind as usize] = 12 + indicies_for_quad * index_cpy;
        self.indices[22 + index_ind as usize] = 15 + indicies_for_quad * index_cpy;
        self.indices[23 + index_ind as usize] = 14 + indicies_for_quad * index_cpy;
        self.indices[24 + index_ind as usize] = 16 + indicies_for_quad * index_cpy;
        self.indices[25 + index_ind as usize] = 17 + indicies_for_quad * index_cpy;
        self.indices[26 + index_ind as usize] = 18 + indicies_for_quad * index_cpy;
        self.indices[27 + index_ind as usize] = 18 + indicies_for_quad * index_cpy;
        self.indices[28 + index_ind as usize] = 19 + indicies_for_quad * index_cpy;
        self.indices[29 + index_ind as usize] = 16 + indicies_for_quad * index_cpy;
        self.indices[30 + index_ind as usize] = 22 + indicies_for_quad * index_cpy;
        self.indices[31 + index_ind as usize] = 21 + indicies_for_quad * index_cpy;
        self.indices[32 + index_ind as usize] = 20 + indicies_for_quad * index_cpy;
        self.indices[33 + index_ind as usize] = 20 + indicies_for_quad * index_cpy;
        self.indices[34 + index_ind as usize] = 23 + indicies_for_quad * index_cpy;
        self.indices[35 + index_ind as usize] = 22 + indicies_for_quad * index_cpy;
        //self.indices = [
        //  2, 1, 0, 0, 3, 2, 5, 6, 7, 7, 4, 5, 8, 9, 10, 10, 11, 8, 14, 13, 12, 12, 15, 14, 16,
        // 17, 18, 18, 19, 16, 22, 21, 20, 20, 23, 22,
        //]
        //.to_vec();
    }

    pub unsafe fn load_cubes(&mut self, pos: math::Vec3) {
        self.load_cube(&pos, 0);
        //self.load_cube(&math::Vec3::new(3.0, 0.0, 0.0), 1);
        self.vao.bind();
        self.vbo.set_data(&self.vertices, gl::STATIC_DRAW);
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
            72,
            gl::UNSIGNED_INT,
            std::ptr::null(),
        );
    }
}
