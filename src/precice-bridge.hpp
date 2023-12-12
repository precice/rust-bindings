#pragma once

#include <memory>
#include <rust/cxx.h>

namespace precice {
class Participant;
}

namespace precice::rust {

using rint = int32_t;

using vid = rint;

class Participant {
public:
  // Construction

  Participant(::rust::Str participant, ::rust::Str config, rint rank, rint size);
  ~Participant();

  // Steering Methods

  void initialize();
  void advance(double dt);
  void finalize();

  // Implicit coupling
  bool requires_writing_checkpoint();
  bool requires_reading_checkpoint();

  // Status Queries
  rint   get_mesh_dimensions(::rust::Str mesh_name) const;
  rint   get_data_dimensions(::rust::Str mesh_name, ::rust::Str data_name) const;
  bool   is_coupling_ongoing() const;
  bool   is_time_window_complete() const;
  double get_max_time_step_size() const;

  // Mesh Access

  bool requires_mesh_connectivity_for(::rust::Str mesh_name) const;
  int  set_mesh_vertex(::rust::Str mesh_name, ::rust::Slice<const double> position);
  int  get_mesh_vertex_size(::rust::Str mesh_name) const;
  void set_mesh_vertices(::rust::Str mesh_name, ::rust::Slice<const double> positions, ::rust::Slice<vid> ids);
  void set_mesh_edge(::rust::Str mesh_name, int first_vertex_id, int second_vertex_id);
  void set_mesh_edges(::rust::Str mesh_name, ::rust::Slice<const int> vertices);
  void set_mesh_triangle(::rust::Str mesh_name, int first_vertex_id, int second_vertex_id, int third_vertex_id);
  void set_mesh_triangles(::rust::Str mesh_name, ::rust::Slice<const int> vertices);
  void set_mesh_quad(::rust::Str mesh_name, int first_vertex_id, int second_vertex_id, int third_vertex_id, int fourth_vertex_id);
  void set_mesh_quads(::rust::Str mesh_name, ::rust::Slice<const int> vertices);
  void set_mesh_tetrahedron(::rust::Str mesh_name, int first_vertex_id, int second_vertex_id, int third_vertex_id, int fourth_vertex_id);
  void set_mesh_tetrahedra(::rust::Str mesh_name, ::rust::Slice<const int> vertices);

  // Data Access

  bool requires_initial_data();
  void write_data(::rust::Str mesh_name, ::rust::Str data_name, ::rust::Slice<const vid> vertices, ::rust::Slice<const double> values);
  void read_data(::rust::Str mesh_name, ::rust::Str data_name, ::rust::Slice<const vid> vertices, double relativeReadTime, ::rust::Slice<double> values) const;

  // experimental: Direct Access

  void set_mesh_access_region(const ::rust::Str mesh_name, ::rust::Slice<const double> boundingBox);
  void get_mesh_vertex_ids_and_coordinates(const ::rust::Str mesh_name, ::rust::Slice<vid> ids, ::rust::Slice<double> coordinates) const;

  // experimental: Gradient Data

  bool requires_gradient_data_for(::rust::Str mesh_name, ::rust::Str data_name) const;
  void write_gradient_data(::rust::Str mesh_name, ::rust::Str data_name, ::rust::Slice<const vid> vertices, ::rust::Slice<const double> gradients);

private:
  std::unique_ptr<::precice::Participant> participant;
};

std::unique_ptr<::precice::rust::Participant> create_participant(::rust::Str participant, ::rust::Str config, rint rank, rint size);

} // namespace precice::rust
