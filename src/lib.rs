//! The preCICE rust bindings.
//!
//! These are the official rust bindings for the [preCICE coupling library](https://precice.org).
//!
//! The methods of the [Participant] map directly to their C++ counterparts.
//! Therefore the documentation of this crate is kept relatively minimal.
//!  
//! Have a deeper look at the official documentation:
//!
//! - The [official documentation](https://precice.org/docs.html) explains what preCICE is and how
//! to use it.
//! - The [API documentation](https://precice.org/dev-docs-sourcedocs.html) explains the API
//! conditions in detail.
//!
//! # Example applications
//!
//! - The crate contains a [solverdummy](https://github.com/precice/rust-bindings/tree/main/examples) as example.
//! - The tutorials contain a rust version of both participants for the [1D elastic-tube tutorial](https://precice.org/tutorials-elastic-tube-1d.html).

#[warn(missing_docs)]
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
        ) -> Result<UniquePtr<Participant>>;

        // Steering Methods
        fn initialize(self: Pin<&mut Participant>) -> Result<()>;
        fn advance(self: Pin<&mut Participant>, dt: f64) -> Result<()>;
        fn finalize(self: Pin<&mut Participant>) -> Result<()>;

        // Implicit coupling
        fn requires_reading_checkpoint(self: Pin<&mut Participant>) -> Result<bool>;
        fn requires_writing_checkpoint(self: Pin<&mut Participant>) -> Result<bool>;

        // Status Queries
        fn get_mesh_dimensions(self: &Participant, mesh_name: &str) -> Result<i32>;
        fn get_data_dimensions(self: &Participant, mesh_name: &str, data_name: &str)
            -> Result<i32>;
        fn is_coupling_ongoing(self: &Participant) -> Result<bool>;
        fn is_time_window_complete(self: &Participant) -> Result<bool>;
        fn get_max_time_step_size(self: &Participant) -> Result<f64>;

        // Mesh Access

        fn requires_mesh_connectivity_for(self: &Participant, mesh_name: &str) -> Result<bool>;
        fn reset_mesh(self: Pin<&mut Participant>, mesh_name: &str) -> Result<()>;
        fn set_mesh_vertex(
            self: Pin<&mut Participant>,
            mesh_name: &str,
            position: &[f64],
        ) -> Result<i32>;
        fn get_mesh_vertex_size(self: &Participant, mesh_name: &str) -> Result<i32>;
        fn set_mesh_vertices(
            self: Pin<&mut Participant>,
            mesh_name: &str,
            positions: &[f64],
            ids: &mut [i32],
        ) -> Result<()>;
        fn set_mesh_edge(
            self: Pin<&mut Participant>,
            mesh_name: &str,
            first_vertex_id: i32,
            second_vertex_id: i32,
        ) -> Result<()>;
        fn set_mesh_edges(
            self: Pin<&mut Participant>,
            mesh_name: &str,
            vertices: &[i32],
        ) -> Result<()>;
        fn set_mesh_triangle(
            self: Pin<&mut Participant>,
            mesh_name: &str,
            first_vertex_id: i32,
            second_vertex_id: i32,
            third_vertex_id: i32,
        ) -> Result<()>;
        fn set_mesh_triangles(
            self: Pin<&mut Participant>,
            mesh_name: &str,
            vertices: &[i32],
        ) -> Result<()>;
        fn set_mesh_quad(
            self: Pin<&mut Participant>,
            mesh_name: &str,
            first_vertex_id: i32,
            second_vertex_id: i32,
            third_vertex_id: i32,
            fourth_vertex_id: i32,
        ) -> Result<()>;
        fn set_mesh_quads(
            self: Pin<&mut Participant>,
            mesh_name: &str,
            vertices: &[i32],
        ) -> Result<()>;
        fn set_mesh_tetrahedron(
            self: Pin<&mut Participant>,
            mesh_name: &str,
            first_vertex_id: i32,
            second_vertex_id: i32,
            third_vertex_id: i32,
            fourth_vertex_id: i32,
        ) -> Result<()>;
        fn set_mesh_tetrahedra(
            self: Pin<&mut Participant>,
            mesh_name: &str,
            vertices: &[i32],
        ) -> Result<()>;

        // Data Access

        fn requires_initial_data(self: Pin<&mut Participant>) -> Result<bool>;

        fn write_data(
            self: Pin<&mut Participant>,
            mesh_name: &str,
            data_name: &str,
            vertices: &[i32],
            values: &[f64],
        ) -> Result<()>;
        fn read_data(
            self: &Participant,
            mesh_name: &str,
            data_name: &str,
            vertices: &[i32],
            relative_read_dt: f64,
            values: &mut [f64],
        ) -> Result<()>;

        // User profiling

        pub fn start_profiling_section(
            self: Pin<&mut Participant>,
            section_name: &str,
        ) -> Result<()>;

        pub fn stop_last_profiling_section(self: Pin<&mut Participant>) -> Result<()>;

        // Direct Access

        fn set_mesh_access_region(
            self: Pin<&mut Participant>,
            mesh_name: &str,
            bounding_box: &[f64],
        ) -> Result<()>;
        fn get_mesh_vertex_ids_and_coordinates(
            self: &Participant,
            mesh_name: &str,
            ids: &mut [i32],
            coordinates: &mut [f64],
        ) -> Result<()>;

        // experimental: Gradient Data

        fn requires_gradient_data_for(
            self: &Participant,
            mesh_name: &str,
            data_name: &str,
        ) -> Result<bool>;
        fn write_gradient_data(
            self: Pin<&mut Participant>,
            mesh_name: &str,
            data_name: &str,
            vertices: &[i32],
            gradients: &[f64],
        ) -> Result<()>;

        // experimental: JIT mapping

        fn write_and_map_data(
            self: Pin<&mut Participant>,
            mesh_name: &str,
            data_name: &str,
            coordinates: &[f64],
            values: &[f64],
        ) -> Result<()>;

        fn map_and_read_data(
            self: &Participant,
            mesh_name: &str,
            data_name: &str,
            coordinates: &[f64],
            relative_read_dt: f64,
            values: &mut [f64],
        ) -> Result<()>;
    }
}

pub struct Participant {
    internal: cxx::UniquePtr<ffi::Participant>,
}

/// Type of vertex identifiers.
///
/// These are returned either by
/// - manually setting vertices from coordinates using [Participant::set_mesh_vertex] and [Participant::set_mesh_vertices], or
/// - using direct api-access with [Participant::get_mesh_vertex_ids_and_coordinates].
pub type VertexID = i32;

/// Type of thrown errors from preCICE
pub type Error = cxx::Exception;

impl Participant {
    // Creates a new preCICE participant.
    //
    // The participant is named `participant` and is descried in the configuration file located at `config`.
    //
    // The communicator size and rank match the MPI terminology.
    // Participants running in serial or not using MPI pass `rank = 0` and `size = 1`.
    pub fn new(
        participant: &str,
        config: &str,
        rank: i32,
        size: i32,
    ) -> Result<Participant, Error> {
        Ok(Participant {
            internal: ffi::create_participant(participant, config, rank, size)?,
        })
    }

    // Steering Methods

    /// Initializes the participant.
    pub fn initialize(&mut self) -> Result<(), Error> {
        self.internal.pin_mut().initialize()
    }

    /// Advances the participant in time by the time-step-size `dt`.
    pub fn advance(&mut self, dt: f64) -> Result<(), Error> {
        self.internal.pin_mut().advance(dt)
    }

    /// Explicitly finalizes the participant, which is not necessary for most use-cases.
    pub fn finalize(&mut self) -> Result<(), Error> {
        self.internal.pin_mut().finalize()
    }

    // Implicit coupling

    /// Checks if the solver needs to read an iteration checkpoint.
    pub fn requires_reading_checkpoint(&mut self) -> Result<bool, Error> {
        self.internal.pin_mut().requires_reading_checkpoint()
    }

    /// Checks if the solver needs to write an iteration checkpoint.
    pub fn requires_writing_checkpoint(&mut self) -> Result<bool, Error> {
        self.internal.pin_mut().requires_writing_checkpoint()
    }

    // Status Queries

    /// Returns the spatial dimensions of a mesh.
    pub fn get_mesh_dimensions(&self, mesh_name: &str) -> Result<i32, Error> {
        self.internal.get_mesh_dimensions(mesh_name)
    }

    /// Returns the spatial dimensions of a data defined on a mesh.
    pub fn get_data_dimensions(&self, mesh_name: &str, data_name: &str) -> Result<i32, Error> {
        self.internal.get_data_dimensions(mesh_name, data_name)
    }

    /// Checks is the coupling is still ongoing .
    pub fn is_coupling_ongoing(&self) -> Result<bool, Error> {
        self.internal.is_coupling_ongoing()
    }

    /// Checks if the previous step completed the time window.
    pub fn is_time_window_complete(&self) -> Result<bool, Error> {
        self.internal.is_time_window_complete()
    }

    /// Returns the maximum time-step-size allowed by a solver at the moment of calling.
    ///
    /// Note that this can return [f64::INFINITY] if the participant prescribes the time-window-size
    /// in a serial coupling scheme.
    pub fn get_max_time_step_size(&self) -> Result<f64, Error> {
        self.internal.get_max_time_step_size()
    }

    // Mesh Access

    /// Check if the mesh makes use of mesh connectivity.
    pub fn requires_mesh_connectivity_for(&self, mesh_name: &str) -> Result<bool, Error> {
        self.internal.requires_mesh_connectivity_for(mesh_name)
    }

    /// Deletes all vertices of a mesh.
    pub fn reset_mesh(&mut self, mesh_name: &str) -> Result<(), Error> {
        self.internal.pin_mut().reset_mesh(mesh_name)
    }

    /// Returns the amount of vertices of a mesh.
    ///
    /// For a provided mesh, returns the amount of vertices.
    ///
    /// For a received mesh using API-access, returns the amount of vertices received.
    /// This is necessary to allocate output vectors for [Participant::get_mesh_vertex_ids_and_coordinates]
    pub fn get_mesh_vertex_size(&self, mesh_name: &str) -> Result<i32, Error> {
        self.internal.get_mesh_vertex_size(mesh_name)
    }

    /// Add a vertex of given position to a mesh and returns a [VertexID].
    pub fn set_mesh_vertex(
        &mut self,
        mesh_name: &str,
        position: &[f64],
    ) -> Result<VertexID, Error> {
        self.internal.pin_mut().set_mesh_vertex(mesh_name, position)
    }

    /// Add multiple vertices of given positions to a mesh and writes their [VertexID]s to `ids` .
    pub fn set_mesh_vertices(
        &mut self,
        mesh_name: &str,
        positions: &[f64],
        ids: &mut [VertexID],
    ) -> Result<(), Error> {
        self.internal
            .pin_mut()
            .set_mesh_vertices(mesh_name, positions, ids)
    }

    /// Defines an edge between two vertices.
    pub fn set_mesh_edge(
        &mut self,
        mesh_name: &str,
        first_vertex_id: VertexID,
        second_vertex_id: VertexID,
    ) -> Result<(), Error> {
        self.internal
            .pin_mut()
            .set_mesh_edge(mesh_name, first_vertex_id, second_vertex_id)
    }

    /// Defines edges between multiple vertex pairs.
    pub fn set_mesh_edges(&mut self, mesh_name: &str, vertices: &[VertexID]) -> Result<(), Error> {
        self.internal.pin_mut().set_mesh_edges(mesh_name, vertices)
    }

    /// Defines a triangle between three vertices.
    pub fn set_mesh_triangle(
        &mut self,
        mesh_name: &str,
        first_vertex_id: VertexID,
        second_vertex_id: VertexID,
        third_vertex_id: VertexID,
    ) -> Result<(), Error> {
        self.internal.pin_mut().set_mesh_triangle(
            mesh_name,
            first_vertex_id,
            second_vertex_id,
            third_vertex_id,
        )
    }

    /// Defines triangles between vertex triples.
    pub fn set_mesh_triangles(
        &mut self,
        mesh_name: &str,
        vertices: &[VertexID],
    ) -> Result<(), Error> {
        self.internal
            .pin_mut()
            .set_mesh_triangles(mesh_name, vertices)
    }

    /// Defines a planar quad between four vertices.
    pub fn set_mesh_quad(
        &mut self,
        mesh_name: &str,
        first_vertex_id: VertexID,
        second_vertex_id: VertexID,
        third_vertex_id: VertexID,
        fourth_vertex_id: VertexID,
    ) -> Result<(), Error> {
        self.internal.pin_mut().set_mesh_quad(
            mesh_name,
            first_vertex_id,
            second_vertex_id,
            third_vertex_id,
            fourth_vertex_id,
        )
    }

    /// Defines multiple planar quads between vertex quadruples.
    pub fn set_mesh_quads(&mut self, mesh_name: &str, vertices: &[VertexID]) -> Result<(), Error> {
        self.internal.pin_mut().set_mesh_quads(mesh_name, vertices)
    }

    /// Defines a tetrahedron between four vertices.
    pub fn set_mesh_tetrahedron(
        &mut self,
        mesh_name: &str,
        first_vertex_id: VertexID,
        second_vertex_id: VertexID,
        third_vertex_id: VertexID,
        fourth_vertex_id: VertexID,
    ) -> Result<(), Error> {
        self.internal.pin_mut().set_mesh_tetrahedron(
            mesh_name,
            first_vertex_id,
            second_vertex_id,
            third_vertex_id,
            fourth_vertex_id,
        )
    }

    /// Defines multiple tetrahedra between vertex quadruples.
    pub fn set_mesh_tetrahedra(
        &mut self,
        mesh_name: &str,
        vertices: &[VertexID],
    ) -> Result<(), Error> {
        self.internal
            .pin_mut()
            .set_mesh_tetrahedra(mesh_name, vertices)
    }

    // Data Access

    /// Checks if this participant needs to provide initial data using [Participant::write_data]
    /// before calling [Participant::initialize].
    pub fn requires_initial_data(&mut self) -> Result<bool, Error> {
        self.internal.pin_mut().requires_initial_data()
    }

    /// Writes data to a mesh for given [VertexID]s.
    pub fn write_data(
        &mut self,
        mesh_name: &str,
        data_name: &str,
        vertices: &[VertexID],
        values: &[f64],
    ) -> Result<(), Error> {
        self.internal
            .pin_mut()
            .write_data(mesh_name, data_name, vertices, values)
    }

    /// Reads data values from a mesh for given [VertexID]s at a given `relative_read_dt`.
    pub fn read_data(
        &self,
        mesh_name: &str,
        data_name: &str,
        vertices: &[VertexID],
        relative_read_dt: f64,
        values: &mut [f64],
    ) -> Result<(), Error> {
        self.internal
            .read_data(mesh_name, data_name, vertices, relative_read_dt, values)
    }

    // User Profiling

    /// Starts a new user-defined profiling section.
    pub fn start_profiling_section(&mut self, section_name: &str) -> Result<(), Error> {
        self.internal
            .pin_mut()
            .start_profiling_section(section_name)
    }

    /// Stops the last user-defined profiling section.
    pub fn stop_last_profiling_section(&mut self) -> Result<(), Error> {
        self.internal.pin_mut().stop_last_profiling_section()
    }

    // Direct Access

    /// Defines a region of interest on a received mesh.
    pub fn set_mesh_access_region(
        &mut self,
        mesh_name: &str,
        bounding_box: &[f64],
    ) -> Result<(), Error> {
        self.internal
            .pin_mut()
            .set_mesh_access_region(mesh_name, bounding_box)
    }

    /// Reads [VertexID]s and coordinates of a received mesh.
    ///
    /// Use [Participant::get_mesh_vertex_size] and [Participant::get_mesh_dimensions] to
    /// preallocate the output storage.
    pub fn get_mesh_vertex_ids_and_coordinates(
        &self,
        mesh_name: &str,
        ids: &mut [VertexID],
        coordinates: &mut [f64],
    ) -> Result<(), Error> {
        self.internal
            .get_mesh_vertex_ids_and_coordinates(mesh_name, ids, coordinates)
    }

    // experimental: Gradient Data

    /// Checks if the given data set requires gradient data.
    pub fn requires_gradient_data_for(
        &self,
        mesh_name: &str,
        data_name: &str,
    ) -> Result<bool, Error> {
        self.internal
            .requires_gradient_data_for(mesh_name, data_name)
    }

    /// Writes vector gradient data to a mesh.
    ///
    /// The used format for `gradients` follows this pattern:
    ///
    /// | Spatial Dimensions | Scalar Data | Vectorial Data |
    /// | --- | --- | --- |
    /// | **2D** | s dx, s dy | x dx, y dx, x dy, y dy |
    /// | **3D** | s dy, s dy, s dz | x dx, y dx, z dx, x dy, y dy, z dy, x dz, y dz, z dz |
    pub fn write_gradient_data(
        &mut self,
        mesh_name: &str,
        data_name: &str,
        vertices: &[VertexID],
        gradients: &[f64],
    ) -> Result<(), Error> {
        self.internal
            .pin_mut()
            .write_gradient_data(mesh_name, data_name, vertices, gradients)
    }

    // experimental: JIT mapping

    /// Reads data values from a mesh using a just-in-time data mapping.
    ///
    /// Values correspond to a given point in time relative to the beginning of the current time-step.
    pub fn write_and_map_data(
        &mut self,
        mesh_name: &str,
        data_name: &str,
        coordinates: &[f64],
        values: &[f64],
    ) -> Result<(), Error> {
        self.internal
            .pin_mut()
            .write_and_map_data(mesh_name, data_name, coordinates, values)
    }

    /// Writes data values to a mesh using a just-in-time mapping (experimental).
    pub fn map_and_read_data(
        &self,
        mesh_name: &str,
        data_name: &str,
        coordinates: &[f64],
        relative_read_dt: f64,
        values: &mut [f64],
    ) -> Result<(), Error> {
        self.internal
            .map_and_read_data(mesh_name, data_name, coordinates, relative_read_dt, values)
    }
}
