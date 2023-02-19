#pragma once

#include <memory>
#include <rust/cxx.h>

namespace precice {
class SolverInterface;
}

namespace precice::rust {

using rint = int32_t;

class SolverInterface {
public:
  // Construction

  SolverInterface(::rust::Str participant, ::rust::Str config, rint rank, rint size);
  ~SolverInterface();

  // Steering Methods

  double advance(double dt);
  void   initialize_data();
  double initialize();
  void   finalize();

  // Status Queries

  rint get_dimensions() const;
  bool is_coupling_ongoing() const;
  bool is_time_window_complete() const;

  // Action Methods

  void mark_action_fulfilled(::rust::Str action);
  bool is_action_required(::rust::Str action) const;

  // Mesh Access

  bool has_mesh(::rust::Str mesh_name) const;
  rint get_mesh_id(::rust::Str mesh_name) const;
  bool is_mesh_connectivity_required(rint mesh_id) const;
  rint set_mesh_vertex(rint mesh_id, ::rust::Slice<const double> position);
  rint get_mesh_vertex_size(rint mesh_id) const;
  void set_mesh_vertices(rint mesh_id, ::rust::Slice<const double> positions, ::rust::Slice<rint> ids);
  rint set_mesh_edge(rint mesh_id, int first_vertex_id, int second_vertex_id);
  void set_mesh_triangle(rint mesh_id, int first_edge_id, int second_edge_id, int third_edge_id);
  void set_mesh_triangle_with_edges(rint mesh_id, int first_vertex_id, int second_vertex_id, int third_vertex_id);
  void set_mesh_quad(rint mesh_id, int first_edge_id, int second_edge_id, int third_edge_id, int fourth_edge_id);
  void set_mesh_quad_with_edges(rint mesh_id, int first_vertex_id, int second_vertex_id, int third_vertex_id, int fourth_vertex_id);
  void set_mesh_tetrahedron(rint mesh_id, int first_vertex_id, int second_vertex_id, int third_vertex_id, int fourth_vertex_id);

  // Data Access

  bool has_data(::rust::Str dataName, rint mesh_id) const;
  rint get_data_id(::rust::Str dataName, rint mesh_id) const;
  void write_block_vector_data(rint data_id, ::rust::Slice<const rint> valueIndices, ::rust::Slice<const double> values);
  void write_vector_data(rint data_id, rint value_index, ::rust::Slice<const double> value);
  void write_block_scalar_data(rint data_id, ::rust::Slice<const rint> valueIndices, ::rust::Slice<const double> values);
  void write_scalar_data(rint data_id, rint value_index, double value);
  void read_block_vector_data(rint data_id, ::rust::Slice<const rint> valueIndices, ::rust::Slice<double> values) const;
  void read_vector_data(rint data_id, rint value_index, ::rust::Slice<double> value) const;
  void read_block_scalar_data(rint data_id, ::rust::Slice<const rint> valueIndices, ::rust::Slice<double> values) const;
  void read_scalar_data(rint data_id, rint value_index, double &value) const;

  // experimental: Direct Access

  void set_mesh_access_region(const rint mesh_id, ::rust::Slice<const double> boundingBox);
  void get_mesh_vertices_and_ids(const rint mesh_id, ::rust::Slice<rint> ids, ::rust::Slice<double> coordinates) const;

  // experimental: Time Interpolation

  void read_block_vector_data_rt(rint data_id, ::rust::Slice<const rint> valueIndices, double relativeReadTime, ::rust::Slice<double> values) const;
  void read_vector_data_rt(rint data_id, rint value_index, double relativeReadTime, ::rust::Slice<double> value) const;
  void read_block_scalar_data_rt(rint data_id, ::rust::Slice<const rint> valueIndices, double relativeReadTime, ::rust::Slice<double> values) const;
  void read_scalar_data_rt(rint data_id, rint value_index, double relativeReadTime, double &value) const;

  // experimental: Gradient Data

  bool is_gradient_data_required(rint data_id) const;
  void write_block_vector_gradient_data(rint data_id, ::rust::Slice<const rint> valueIndices, ::rust::Slice<const double> gradient_values);
  void write_scalar_gradient_data(rint data_id, rint value_index, ::rust::Slice<const double> gradient_values);
  void write_vector_gradient_data(rint data_id, rint value_index, ::rust::Slice<const double> gradient_values);
  void write_block_scalar_gradient_data(rint data_id, ::rust::Slice<const rint> valueIndices, ::rust::Slice<const double> gradient_values);

private:
  std::unique_ptr<::precice::SolverInterface> interface;
};

::rust::String action_write_initial_data();
::rust::String action_write_iteration_checkpoint();
::rust::String action_read_iteration_checkpoint();

std::unique_ptr<::precice::rust::SolverInterface> create_solverinterface(::rust::Str participant, ::rust::Str config, rint rank, rint size);

} // namespace precice::rust
