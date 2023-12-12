#[cxx::bridge(namespace = "precice::rust")]
mod ffi {
    unsafe extern "C++" {
        include!("precice/src/precice-bridge.hpp");
        type Participant;

        fn create_participant(
            participant: &str,
            config: &str,
            rank: i32,
            size: i32,
        ) -> UniquePtr<Participant>;

        // Steering Methods
        fn initialize(self: Pin<&mut Participant>);
        fn advance(self: Pin<&mut Participant>, dt: f64);
        fn finalize(self: Pin<&mut Participant>);

        // Implicit coupling
        fn requires_reading_checkpoint(self: Pin<&mut Participant>) -> bool;
        fn requires_writing_checkpoint(self: Pin<&mut Participant>) -> bool;

        // Status Queries
        fn get_mesh_dimensions(self: &Participant, mesh_name: &str) -> i32;
        fn get_data_dimensions(self: &Participant, mesh_name: &str, data_name: &str) -> i32;
        fn is_coupling_ongoing(self: &Participant) -> bool;
        fn is_time_window_complete(self: &Participant) -> bool;
        fn get_max_time_step_size(self: &Participant) -> f64;

        // Mesh Access

        fn requires_mesh_connectivity_for(self: &Participant, mesh_name: &str) -> bool;
        fn set_mesh_vertex(self: Pin<&mut Participant>, mesh_name: &str, position: &[f64]) -> i32;
        fn get_mesh_vertex_size(self: &Participant, mesh_name: &str) -> i32;
        fn set_mesh_vertices(
            self: Pin<&mut Participant>,
            mesh_name: &str,
            positions: &[f64],
            ids: &mut [i32],
        );
        fn set_mesh_edge(
            self: Pin<&mut Participant>,
            mesh_name: &str,
            first_vertex_id: i32,
            second_vertex_id: i32,
        );
        fn set_mesh_edges(self: Pin<&mut Participant>, mesh_name: &str, vertices: &[i32]);
        fn set_mesh_triangle(
            self: Pin<&mut Participant>,
            mesh_name: &str,
            first_vertex_id: i32,
            second_vertex_id: i32,
            third_vertex_id: i32,
        );
        fn set_mesh_triangles(self: Pin<&mut Participant>, mesh_name: &str, vertices: &[i32]);
        fn set_mesh_quad(
            self: Pin<&mut Participant>,
            mesh_name: &str,
            first_vertex_id: i32,
            second_vertex_id: i32,
            third_vertex_id: i32,
            fourth_vertex_id: i32,
        );
        fn set_mesh_quads(self: Pin<&mut Participant>, mesh_name: &str, vertices: &[i32]);
        fn set_mesh_tetrahedron(
            self: Pin<&mut Participant>,
            mesh_name: &str,
            first_vertex_id: i32,
            second_vertex_id: i32,
            third_vertex_id: i32,
            fourth_vertex_id: i32,
        );
        fn set_mesh_tetrahedra(self: Pin<&mut Participant>, mesh_name: &str, vertices: &[i32]);

        // Data Access

        fn requires_initial_data(self: Pin<&mut Participant>) -> bool;

        fn write_data(
            self: Pin<&mut Participant>,
            mesh_name: &str,
            data_name: &str,
            vertices: &[i32],
            values: &[f64],
        );
        fn read_data(
            self: &Participant,
            mesh_name: &str,
            data_name: &str,
            vertices: &[i32],
            relative_read_dt: f64,
            values: &mut [f64],
        );

        // Direct Access

        fn set_mesh_access_region(
            self: Pin<&mut Participant>,
            mesh_name: &str,
            boundingBox: &[f64],
        );
        fn get_mesh_vertex_ids_and_coordinates(
            self: &Participant,
            mesh_name: &str,
            ids: &mut [i32],
            coordinates: &mut [f64],
        );

        // experimental: Gradient Data

        fn requires_gradient_data_for(self: &Participant, mesh_name: &str, data_name: &str)
            -> bool;
        fn write_gradient_data(
            self: Pin<&mut Participant>,
            mesh_name: &str,
            data_name: &str,
            vertices: &[i32],
            gradients: &[f64],
        );
    }
}

// Publish the bindings
pub use ffi::*;

// Publish a rust-like new function
pub use ffi::create_participant as new;
