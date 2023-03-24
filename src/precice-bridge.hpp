#pragma once

#include <memory>
#include <rust/cxx.h>

namespace precice {
class SolverInterface;
}

namespace precice::rust {

using rint = int32_t;

using vid = rint;

class SolverInterface {
public:
  // Construction

  SolverInterface(::rust::Str participant, ::rust::Str config, rint rank, rint size);
  ~SolverInterface();

  // Steering Methods

  double advance(double dt);
  double initialize();
  void   finalize();

  // Status Queries

  rint get_dimensions() const;
  bool is_coupling_ongoing() const;
  bool is_time_window_complete() const;

  // Action Methods

  bool requires_initial_data();
  bool requires_writing_checkpoint();
  bool requires_reading_checkpoint();

  // Mesh Access

  bool has_mesh(::rust::Str mesh_name) const;
  bool requires_mesh_connectivity_for(::rust::Str mesh_name) const;
  vid  set_mesh_vertex(::rust::Str mesh_name, ::rust::Slice<const double> position);
  vid  get_mesh_vertex_size(::rust::Str mesh_name) const;
  void set_mesh_vertices(::rust::Str mesh_name, ::rust::Slice<const double> positions, ::rust::Slice<vid> ids);
  void set_mesh_edge(::rust::Str mesh_name, int first_vertex_id, int second_vertex_id);
  void set_mesh_triangle(::rust::Str mesh_name, int first_vertex_id, int second_vertex_id, int third_vertex_id);
  void set_mesh_quad(::rust::Str mesh_name, int first_vertex_id, int second_vertex_id, int third_vertex_id, int fourth_vertex_id);
  void set_mesh_tetrahedron(::rust::Str mesh_name, int first_vertex_id, int second_vertex_id, int third_vertex_id, int fourth_vertex_id);

  // Data Access

  bool has_data(::rust::Str dataName, ::rust::Str mesh_name) const;
  void write_block_vector_data(::rust::Str mesh_name, ::rust::Str data_name, ::rust::Slice<const vid> valueIndices, ::rust::Slice<const double> values);
  void write_vector_data(::rust::Str mesh_name, ::rust::Str data_name, vid value_index, ::rust::Slice<const double> value);
  void write_block_scalar_data(::rust::Str mesh_name, ::rust::Str data_name, ::rust::Slice<const vid> valueIndices, ::rust::Slice<const double> values);
  void write_scalar_data(::rust::Str mesh_name, ::rust::Str data_name, vid value_index, double value);
  void read_block_vector_data(::rust::Str mesh_name, ::rust::Str data_name, ::rust::Slice<const vid> valueIndices, ::rust::Slice<double> values) const;
  void read_vector_data(::rust::Str mesh_name, ::rust::Str data_name, vid value_index, ::rust::Slice<double> value) const;
  void read_block_scalar_data(::rust::Str mesh_name, ::rust::Str data_name, ::rust::Slice<const vid> valueIndices, ::rust::Slice<double> values) const;
  void read_scalar_data(::rust::Str mesh_name, ::rust::Str data_name, vid value_index, double &value) const;

  // experimental: Direct Access

  void set_mesh_access_region(const ::rust::Str mesh_name, ::rust::Slice<const double> boundingBox);
  void get_mesh_vertices_and_ids(const ::rust::Str mesh_name, ::rust::Slice<vid> ids, ::rust::Slice<double> coordinates) const;

  // experimental: Time Interpolation

  void read_block_vector_data_rt(::rust::Str mesh_name, ::rust::Str data_name, ::rust::Slice<const vid> valueIndices, double relativeReadTime, ::rust::Slice<double> values) const;
  void read_vector_data_rt(::rust::Str mesh_name, ::rust::Str data_name, vid value_index, double relativeReadTime, ::rust::Slice<double> value) const;
  void read_block_scalar_data_rt(::rust::Str mesh_name, ::rust::Str data_name, ::rust::Slice<const vid> valueIndices, double relativeReadTime, ::rust::Slice<double> values) const;
  void read_scalar_data_rt(::rust::Str mesh_name, ::rust::Str data_name, vid value_index, double relativeReadTime, double &value) const;

  // experimental: Gradient Data

  bool requires_gradient_data_for(::rust::Str mesh_name, ::rust::Str data_name) const;
  void write_block_vector_gradient_data(::rust::Str mesh_name, ::rust::Str data_name, ::rust::Slice<const vid> valueIndices, ::rust::Slice<const double> gradient_values);
  void write_scalar_gradient_data(::rust::Str mesh_name, ::rust::Str data_name, vid value_index, ::rust::Slice<const double> gradient_values);
  void write_vector_gradient_data(::rust::Str mesh_name, ::rust::Str data_name, vid value_index, ::rust::Slice<const double> gradient_values);
  void write_block_scalar_gradient_data(::rust::Str mesh_name, ::rust::Str data_name, ::rust::Slice<const vid> valueIndices, ::rust::Slice<const double> gradient_values);

private:
  std::unique_ptr<::precice::SolverInterface> interface;
};

std::unique_ptr<::precice::rust::SolverInterface> create_solverinterface(::rust::Str participant, ::rust::Str config, rint rank, rint size);

} // namespace precice::rust
