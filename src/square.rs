extern crate glium;

use glium::Display;
use glium::vertex::VertexBufferAny;
use glium::vertex::VertexBuffer;
use glium::index::IndexBuffer;
use glium::index::TrianglesList;

use cell::Cell;

pub fn geometry(display: &Display, size: f32) -> (VertexBufferAny, IndexBuffer) {
    (vertices(display, size), indices(display))
}

fn vertices(display: &Display, size: f32) -> VertexBufferAny {
    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
        color: [f32; 3],
    }

    implement_vertex!(Vertex, position, color);

    let colour = [0.2, 0.2, 0.2];

    VertexBuffer::new(display,
        vec![
            Vertex { position: [  0.0,  0.0], color: colour },
            Vertex { position: [  0.0, size], color: colour },
            Vertex { position: [ size, size], color: colour },
            Vertex { position: [ size,  0.0], color: colour },
        ]
    ).into_vertex_buffer_any()
}

fn indices(display: &Display) -> IndexBuffer {
    IndexBuffer::new(display, TrianglesList(vec![0u16, 1, 2, 0, 2, 3]))
}

pub fn instances(display: &Display, grid: &Vec<Cell>) -> VertexBuffer<Location> {
    implement_vertex!(Location, world_position);

    let mut data = Vec::new();
    for cell in grid.iter().filter(|c| c.alive) {
        data.push(Location {
            world_position: [cell.x, cell.y]
        })
    }

    VertexBuffer::new(display, data)
}

#[derive(Copy, Clone)]
pub struct Location {
    world_position: [f32; 2],
}