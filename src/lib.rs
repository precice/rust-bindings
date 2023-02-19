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
        fn initialize_data(self: Pin<&mut SolverInterface>);

        // Status Queries

        fn get_dimensions(&self) -> i32;
        fn is_coupling_ongoing(&self) -> bool;
        fn is_time_window_complete(&self) -> bool;

        // Action Methods

        fn mark_action_fulfilled(self: Pin<&mut SolverInterface>, action: &str);
        fn is_action_required(&self, action: &str) -> bool;

        // Mesh Access

        fn has_mesh(&self, mesh_name: &str) -> bool;
        fn get_mesh_id(&self, mesh_name: &str) -> i32;
        fn is_mesh_connectivity_required(&self, mesh_id: i32) -> bool;
        fn set_mesh_vertex(self: Pin<&mut SolverInterface>, mesh_id: i32, position: &[f64]) -> i32;
        fn get_mesh_vertex_size(&self, mesh_id: i32) -> i32;
        fn set_mesh_vertices(
            self: Pin<&mut SolverInterface>,
            mesh_id: i32,
            positions: &[f64],
            ids: &mut [i32],
        );
        fn set_mesh_edge(
            self: Pin<&mut SolverInterface>,
            mesh_id: i32,
            first_vertex_id: i32,
            second_vertex_id: i32,
        ) -> i32;
        fn set_mesh_triangle(
            self: Pin<&mut SolverInterface>,
            mesh_id: i32,
            first_edge_id: i32,
            second_edge_id: i32,
            third_edge_id: i32,
        );
        fn set_mesh_triangle_with_edges(
            self: Pin<&mut SolverInterface>,
            mesh_id: i32,
            first_vertex_id: i32,
            second_vertex_id: i32,
            third_vertex_id: i32,
        );
        fn set_mesh_quad(
            self: Pin<&mut SolverInterface>,
            mesh_id: i32,
            first_edge_id: i32,
            second_edge_id: i32,
            third_edge_id: i32,
            fourth_edge_id: i32,
        );
        fn set_mesh_quad_with_edges(
            self: Pin<&mut SolverInterface>,
            mesh_id: i32,
            first_vertex_id: i32,
            second_vertex_id: i32,
            third_vertex_id: i32,
            fourth_vertex_id: i32,
        );
        fn set_mesh_tetrahedron(
            self: Pin<&mut SolverInterface>,
            mesh_id: i32,
            first_vertex_id: i32,
            second_vertex_id: i32,
            third_vertex_id: i32,
            fourth_vertex_id: i32,
        );

        // Data Access

        fn has_data(&self, dataName: &str, mesh_id: i32) -> bool;
        fn get_data_id(&self, dataName: &str, mesh_id: i32) -> i32;
        fn write_block_vector_data(
            self: Pin<&mut SolverInterface>,
            data_id: i32,
            valueIndices: &[i32],
            values: &[f64],
        );
        fn write_vector_data(
            self: Pin<&mut SolverInterface>,
            data_id: i32,
            value_index: i32,
            value: &[f64],
        );
        fn write_block_scalar_data(
            self: Pin<&mut SolverInterface>,
            data_id: i32,
            valueIndices: &[i32],
            values: &[f64],
        );
        fn write_scalar_data(
            self: Pin<&mut SolverInterface>,
            data_id: i32,
            value_index: i32,
            value: f64,
        );
        fn read_block_vector_data(&self, data_id: i32, valueIndices: &[i32], values: &mut [f64]);
        fn read_vector_data(&self, data_id: i32, value_index: i32, value: &mut [f64]);
        fn read_block_scalar_data(&self, data_id: i32, valueIndices: &[i32], values: &mut [f64]);
        fn read_scalar_data(&self, data_id: i32, value_index: i32, value: &mut f64);

        // experimental: Direct Access

        fn set_mesh_access_region(
            self: Pin<&mut SolverInterface>,
            mesh_id: i32,
            boundingBox: &[f64],
        );
        fn get_mesh_vertices_and_ids(&self, mesh_id: i32, ids: &mut [i32], coordinates: &mut [f64]);

        // experimental: Time Interpolation

        fn read_block_vector_data_rt(
            &self,
            data_id: i32,
            valueIndices: &[i32],
            relativeReadTime: f64,
            values: &mut [f64],
        );
        fn read_vector_data_rt(
            &self,
            data_id: i32,
            value_index: i32,
            relativeReadTime: f64,
            value: &mut [f64],
        );
        fn read_block_scalar_data_rt(
            &self,
            data_id: i32,
            valueIndices: &[i32],
            relativeReadTime: f64,
            values: &mut [f64],
        );
        fn read_scalar_data_rt(
            &self,
            data_id: i32,
            value_index: i32,
            relativeReadTime: f64,
            value: &mut f64,
        );

        // experimental: Gradient Data

        fn is_gradient_data_required(&self, data_id: i32) -> bool;
        fn write_block_vector_gradient_data(
            self: Pin<&mut SolverInterface>,
            data_id: i32,
            valueIndices: &[i32],
            gradient_values: &[f64],
        );
        fn write_scalar_gradient_data(
            self: Pin<&mut SolverInterface>,
            data_id: i32,
            value_index: i32,
            gradient_values: &[f64],
        );
        fn write_vector_gradient_data(
            self: Pin<&mut SolverInterface>,
            data_id: i32,
            value_index: i32,
            gradient_values: &[f64],
        );
        fn write_block_scalar_gradient_data(
            self: Pin<&mut SolverInterface>,
            data_id: i32,
            valueIndices: &[i32],
            gradient_values: &[f64],
        );

        fn action_write_initial_data() -> String;
        fn action_write_iteration_checkpoint() -> String;
        fn action_read_iteration_checkpoint() -> String;
    }
}

// Publish a rust-like new function
pub use ffi::create_solverinterface as new;

// Publish the action names
pub use ffi::action_read_iteration_checkpoint;
pub use ffi::action_write_initial_data;
pub use ffi::action_write_iteration_checkpoint;
