#[cxx::bridge(namespace = "precice::rust")]
mod ffi {
    unsafe extern "C++" {
        include!("precice/src/precice-bridge.hpp");
        type SolverInterface;

        fn create_solverinterface(
            participant: &str,
            config: &str,
            rank: i32,
            size: i32,
        ) -> UniquePtr<SolverInterface>;

        // Steering Methods

        fn finalize(self: Pin<&mut SolverInterface>);
        fn advance(self: Pin<&mut SolverInterface>, dt: f64) -> f64;
        fn initialize(self: Pin<&mut SolverInterface>) -> f64;

        // Status Queries

        fn get_dimensions(self: &SolverInterface) -> i32;
        fn is_coupling_ongoing(self: &SolverInterface) -> bool;
        fn is_time_window_complete(self: &SolverInterface) -> bool;

        // Action Methods

        fn requires_initial_data(self: Pin<&mut SolverInterface>) -> bool;
        fn requires_reading_checkpoint(self: Pin<&mut SolverInterface>) -> bool;
        fn requires_writing_checkpoint(self: Pin<&mut SolverInterface>) -> bool;

        // Mesh Access

        fn has_mesh(self: &SolverInterface, mesh_name: &str) -> bool;
        fn requires_mesh_connectivity_for(self: &SolverInterface, mesh_name: &str) -> bool;
        fn set_mesh_vertex(
            self: Pin<&mut SolverInterface>,
            mesh_name: &str,
            position: &[f64],
        ) -> i32;
        fn get_mesh_vertex_size(self: &SolverInterface, mesh_name: &str) -> i32;
        fn set_mesh_vertices(
            self: Pin<&mut SolverInterface>,
            mesh_name: &str,
            positions: &[f64],
            ids: &mut [i32],
        );
        fn set_mesh_edge(
            self: Pin<&mut SolverInterface>,
            mesh_name: &str,
            first_vertex_id: i32,
            second_vertex_id: i32,
        );
        fn set_mesh_triangle(
            self: Pin<&mut SolverInterface>,
            mesh_name: &str,
            first_vertex_id: i32,
            second_vertex_id: i32,
            third_vertex_id: i32,
        );
        fn set_mesh_quad(
            self: Pin<&mut SolverInterface>,
            mesh_name: &str,
            first_vertex_id: i32,
            second_vertex_id: i32,
            third_vertex_id: i32,
            fourth_vertex_id: i32,
        );
        fn set_mesh_tetrahedron(
            self: Pin<&mut SolverInterface>,
            mesh_name: &str,
            first_vertex_id: i32,
            second_vertex_id: i32,
            third_vertex_id: i32,
            fourth_vertex_id: i32,
        );

        // Data Access

        fn has_data(self: &SolverInterface, dataName: &str, mesh_name: &str) -> bool;
        fn write_block_vector_data(
            self: Pin<&mut SolverInterface>,
            mesh_name: &str,
            data_name: &str,
            valueIndices: &[i32],
            values: &[f64],
        );
        fn write_vector_data(
            self: Pin<&mut SolverInterface>,
            mesh_name: &str,
            data_name: &str,
            value_index: i32,
            value: &[f64],
        );
        fn write_block_scalar_data(
            self: Pin<&mut SolverInterface>,
            mesh_name: &str,
            data_name: &str,
            valueIndices: &[i32],
            values: &[f64],
        );
        fn write_scalar_data(
            self: Pin<&mut SolverInterface>,
            mesh_name: &str,
            data_name: &str,
            value_index: i32,
            value: f64,
        );
        fn read_block_vector_data(
            self: &SolverInterface,
            mesh_name: &str,
            data_name: &str,
            valueIndices: &[i32],
            values: &mut [f64],
        );
        fn read_vector_data(
            self: &SolverInterface,
            mesh_name: &str,
            data_name: &str,
            value_index: i32,
            value: &mut [f64],
        );
        fn read_block_scalar_data(
            self: &SolverInterface,
            mesh_name: &str,
            data_name: &str,
            valueIndices: &[i32],
            values: &mut [f64],
        );
        fn read_scalar_data(
            self: &SolverInterface,
            mesh_name: &str,
            data_name: &str,
            value_index: i32,
            value: &mut f64,
        );

        // experimental: Direct Access

        fn set_mesh_access_region(
            self: Pin<&mut SolverInterface>,
            mesh_name: &str,
            boundingBox: &[f64],
        );
        fn get_mesh_vertices_and_ids(
            self: &SolverInterface,
            mesh_name: &str,
            ids: &mut [i32],
            coordinates: &mut [f64],
        );

        // experimental: Time Interpolation

        fn read_block_vector_data_rt(
            self: &SolverInterface,
            mesh_name: &str,
            data_name: &str,
            valueIndices: &[i32],
            relativeReadTime: f64,
            values: &mut [f64],
        );
        fn read_vector_data_rt(
            self: &SolverInterface,
            mesh_name: &str,
            data_name: &str,
            value_index: i32,
            relativeReadTime: f64,
            value: &mut [f64],
        );
        fn read_block_scalar_data_rt(
            self: &SolverInterface,
            mesh_name: &str,
            data_name: &str,
            valueIndices: &[i32],
            relativeReadTime: f64,
            values: &mut [f64],
        );
        fn read_scalar_data_rt(
            self: &SolverInterface,
            mesh_name: &str,
            data_name: &str,
            value_index: i32,
            relativeReadTime: f64,
            value: &mut f64,
        );

        // experimental: Gradient Data

        fn requires_gradient_data_for(
            self: &SolverInterface,
            mesh_name: &str,
            data_name: &str,
        ) -> bool;
        fn write_block_vector_gradient_data(
            self: Pin<&mut SolverInterface>,
            mesh_name: &str,
            data_name: &str,
            valueIndices: &[i32],
            gradient_values: &[f64],
        );
        fn write_scalar_gradient_data(
            self: Pin<&mut SolverInterface>,
            mesh_name: &str,
            data_name: &str,
            value_index: i32,
            gradient_values: &[f64],
        );
        fn write_vector_gradient_data(
            self: Pin<&mut SolverInterface>,
            mesh_name: &str,
            data_name: &str,
            value_index: i32,
            gradient_values: &[f64],
        );
        fn write_block_scalar_gradient_data(
            self: Pin<&mut SolverInterface>,
            mesh_name: &str,
            data_name: &str,
            valueIndices: &[i32],
            gradient_values: &[f64],
        );
    }
}

// Publish the bindings
pub use ffi::*;

// Publish a rust-like new function
pub use ffi::create_solverinterface as new;
