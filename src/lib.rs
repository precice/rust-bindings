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
    }
}

pub struct Participant {
    internal: cxx::UniquePtr<ffi::Participant>,
}

pub type VertexID = i32;
pub type Error = cxx::Exception;

impl Participant {
    pub fn new(participant: &str, config: &str, rank: i32, size: i32) -> Result<Self, Error> {
        Ok(Participant {
            internal: ffi::create_participant(participant, config, rank, size)?,
        })
    }

    // Steering Methods
    pub fn initialize(&mut self) -> Result<(), Error> {
        self.internal.pin_mut().initialize()
    }
    pub fn advance(&mut self, dt: f64) -> Result<(), Error> {
        self.internal.pin_mut().advance(dt)
    }
    pub fn finalize(&mut self) -> Result<(), Error> {
        self.internal.pin_mut().finalize()
    }

    // Implicit coupling
    pub fn requires_reading_checkpoint(&mut self) -> Result<bool, Error> {
        self.internal.pin_mut().requires_reading_checkpoint()
    }
    pub fn requires_writing_checkpoint(&mut self) -> Result<bool, Error> {
        self.internal.pin_mut().requires_writing_checkpoint()
    }

    // Status Queries
    pub fn get_mesh_dimensions(&self, mesh_name: &str) -> Result<i32, Error> {
        self.internal.get_mesh_dimensions(mesh_name)
    }
    pub fn get_data_dimensions(&self, mesh_name: &str, data_name: &str) -> Result<i32, Error> {
        self.internal.get_data_dimensions(mesh_name, data_name)
    }
    pub fn is_coupling_ongoing(&self) -> Result<bool, Error> {
        self.internal.is_coupling_ongoing()
    }
    pub fn is_time_window_complete(&self) -> Result<bool, Error> {
        self.internal.is_time_window_complete()
    }
    pub fn get_max_time_step_size(&self) -> Result<f64, Error> {
        self.internal.get_max_time_step_size()
    }

    // Mesh Access

    pub fn requires_mesh_connectivity_for(&self, mesh_name: &str) -> Result<bool, Error> {
        self.internal.requires_mesh_connectivity_for(mesh_name)
    }
    pub fn set_mesh_vertex(
        &mut self,
        mesh_name: &str,
        position: &[f64],
    ) -> Result<VertexID, Error> {
        self.internal.pin_mut().set_mesh_vertex(mesh_name, position)
    }
    pub fn get_mesh_vertex_size(&self, mesh_name: &str) -> Result<i32, Error> {
        self.internal.get_mesh_vertex_size(mesh_name)
    }
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
    pub fn set_mesh_edges(&mut self, mesh_name: &str, vertices: &[VertexID]) -> Result<(), Error> {
        self.internal.pin_mut().set_mesh_edges(mesh_name, vertices)
    }
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
    pub fn set_mesh_triangles(
        &mut self,
        mesh_name: &str,
        vertices: &[VertexID],
    ) -> Result<(), Error> {
        self.internal
            .pin_mut()
            .set_mesh_triangles(mesh_name, vertices)
    }
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
    pub fn set_mesh_quads(&mut self, mesh_name: &str, vertices: &[VertexID]) -> Result<(), Error> {
        self.internal.pin_mut().set_mesh_quads(mesh_name, vertices)
    }
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

    pub fn requires_initial_data(&mut self) -> Result<bool, Error> {
        self.internal.pin_mut().requires_initial_data()
    }

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

    // Direct Access

    pub fn set_mesh_access_region(
        &mut self,
        mesh_name: &str,
        bounding_box: &[f64],
    ) -> Result<(), Error> {
        self.internal
            .pin_mut()
            .set_mesh_access_region(mesh_name, bounding_box)
    }
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

    pub fn requires_gradient_data_for(
        &self,
        mesh_name: &str,
        data_name: &str,
    ) -> Result<bool, Error> {
        self.internal
            .requires_gradient_data_for(mesh_name, data_name)
    }
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
}
