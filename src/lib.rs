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

        fn get_dimensions(&self) -> i32;
        fn finalize(self: Pin<&mut SolverInterface>);
        fn mark_action_fulfilled(self: Pin<&mut SolverInterface>, action: &str);
        fn is_action_required(&self, action: &str) -> bool;
        fn advance(self: Pin<&mut SolverInterface>, dt: f64) -> f64;
        fn initialize(self: Pin<&mut SolverInterface>) -> f64;
        fn initialize_data(self: Pin<&mut SolverInterface>);
        fn is_coupling_ongoing(&self) -> bool;

        fn write_block_scalar_data(
            self: Pin<&mut SolverInterface>,
            did: i32,
            vertexIDs: &[i32],
            data: &[f64],
        );
        fn read_block_scalar_data(&self, did: i32, vertexIDs: &[i32], data: &mut [f64]);

        fn set_mesh_vertices(
            self: Pin<&mut SolverInterface>,
            mid: i32,
            coordinates: &[f64],
            vertexIDs: &mut [i32],
        );

        fn is_write_data_required(&self, dt: f64) -> bool;
        fn is_read_data_available(&self) -> bool;

        fn get_mesh_id(&self, name: &str) -> i32;
        fn get_data_id(&self, name: &str, meshID: i32) -> i32;

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
