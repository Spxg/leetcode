#![warn(
    absolute_paths_not_starting_with_crate,
    explicit_outlives_requirements,
    let_underscore_drop,
    macro_use_extern_crate,
    meta_variable_misuse,
    missing_abi,
    missing_docs,
    noop_method_call,
    pointer_structural_match,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unsafe_op_in_unsafe_fn,
    unused_crate_dependencies,
    unused_extern_crates,
    unused_import_braces,
    unused_lifetimes,
    unused_macro_rules,
    unused_qualifications,
    unused_tuple_struct_fields,
    variant_size_differences,
    clippy::alloc_instead_of_core,
    clippy::allow_attributes_without_reason,
    clippy::as_ptr_cast_mut,
    clippy::branches_sharing_code,
    clippy::cargo_common_metadata,
    clippy::clone_on_ref_ptr,
    clippy::cognitive_complexity,
    clippy::create_dir,
    clippy::dbg_macro,
    clippy::debug_assert_with_mut_call,
    clippy::decimal_literal_representation,
    clippy::deref_by_slicing,
    clippy::derive_partial_eq_without_eq,
    clippy::empty_drop,
    clippy::empty_line_after_outer_attr,
    clippy::empty_structs_with_brackets,
    clippy::equatable_if_let,
    clippy::fallible_impl_from,
    clippy::filetype_is_file,
    clippy::float_cmp_const,
    clippy::format_push_string,
    clippy::get_unwrap,
    clippy::if_then_some_else_none,
    clippy::imprecise_flops,
    clippy::iter_on_empty_collections,
    clippy::iter_on_single_items,
    clippy::iter_with_drain,
    clippy::large_include_file,
    clippy::let_underscore_must_use,
    clippy::lossy_float_literal,
    clippy::manual_clamp,
    clippy::map_err_ignore,
    clippy::mixed_read_write_in_expression,
    clippy::multiple_inherent_impl,
    clippy::mutex_atomic,
    clippy::mutex_integer,
    clippy::needless_collect,
    clippy::negative_feature_names,
    clippy::non_send_fields_in_send_ty,
    clippy::nonstandard_macro_braces,
    clippy::option_if_let_else,
    clippy::or_fun_call,
    clippy::panic,
    clippy::panic_in_result_fn,
    clippy::partial_pub_fields,
    clippy::path_buf_push_overwrite,
    clippy::pedantic,
    clippy::print_stderr,
    clippy::print_stdout,
    clippy::rc_buffer,
    clippy::rc_mutex,
    clippy::redundant_feature_names,
    clippy::redundant_pub_crate,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::same_name_method,
    clippy::self_named_module_files,
    clippy::significant_drop_in_scrutinee,
    clippy::string_lit_as_bytes,
    clippy::string_to_string,
    clippy::suboptimal_flops,
    clippy::suspicious_operation_groupings,
    clippy::todo,
    clippy::trailing_empty_array,
    clippy::trait_duplication_in_bounds,
    clippy::transmute_undefined_repr,
    clippy::trivial_regex,
    clippy::try_err,
    clippy::type_repetition_in_bounds,
    clippy::undocumented_unsafe_blocks,
    clippy::unimplemented,
    clippy::unnecessary_safety_comment,
    clippy::unnecessary_safety_doc,
    clippy::unnecessary_self_imports,
    clippy::unneeded_field_pattern,
    clippy::unused_peekable,
    clippy::unused_rounding,
    clippy::use_debug,
    clippy::use_self,
    clippy::useless_let_if_seq,
    clippy::verbose_file_reads,
    clippy::wildcard_dependencies
)]
#![allow(
    missing_docs,
    trivial_casts,
    clippy::cargo_common_metadata,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::decimal_literal_representation,
    clippy::missing_panics_doc,
    clippy::must_use_candidate,
    clippy::needless_pass_by_value,
    clippy::same_name_method,
    clippy::wildcard_dependencies
)]

pub mod data_structures;

pub mod problem_0001_two_sum;
pub mod problem_0002_add_two_numbers;
pub mod problem_0003_longest_substring_without_repeating_characters;
pub mod problem_0005_longest_palindromic_substring;
pub mod problem_0006_zigzag_conversion;
pub mod problem_0007_reverse_integer;
pub mod problem_0008_string_to_integer_atoi;
pub mod problem_0009_palindrome_number;
pub mod problem_0010_regular_expression_matching;
pub mod problem_0011_container_with_most_water;
pub mod problem_0012_integer_to_roman;
pub mod problem_0013_roman_to_integer;
pub mod problem_0014_longest_common_prefix;
pub mod problem_0015_3sum;
pub mod problem_0016_3sum_closest;
pub mod problem_0017_letter_combinations_of_a_phone_number;
pub mod problem_0018_4sum;
pub mod problem_0019_remove_nth_node_from_end_of_list;
pub mod problem_0020_valid_parentheses;
pub mod problem_0021_merge_two_sorted_lists;
pub mod problem_0022_generate_parentheses;
pub mod problem_0023_merge_k_sorted_lists;
pub mod problem_0024_swap_nodes_in_pairs;
pub mod problem_0026_remove_duplicates_from_sorted_array;
pub mod problem_0027_remove_element;
pub mod problem_0028_find_the_index_of_the_first_occurrence_in_a_string;
pub mod problem_0031_next_permutation;
pub mod problem_0033_search_in_rotated_sorted_array;
pub mod problem_0034_find_first_and_last_position_of_element_in_sorted_array;
pub mod problem_0035_search_insert_position;
pub mod problem_0036_valid_sudoku;
pub mod problem_0038_count_and_say;
pub mod problem_0039_combination_sum;
pub mod problem_0040_combination_sum_ii;
pub mod problem_0041_first_missing_positive;
pub mod problem_0043_multiply_strings;
pub mod problem_0045_jump_game_ii;
pub mod problem_0046_permutations;
pub mod problem_0047_permutations_ii;
pub mod problem_0048_rotate_image;
pub mod problem_0049_group_anagrams;
pub mod problem_0050_powx_n;
pub mod problem_0053_maximum_subarray;
pub mod problem_0054_spiral_matrix;
pub mod problem_0055_jump_game;
pub mod problem_0056_merge_intervals;
pub mod problem_0057_insert_interval;
pub mod problem_0058_length_of_last_word;
pub mod problem_0059_spiral_matrix_ii;
pub mod problem_0061_rotate_list;
pub mod problem_0062_unique_paths;
pub mod problem_0063_unique_paths_ii;
pub mod problem_0064_minimum_path_sum;
pub mod problem_0066_plus_one;
pub mod problem_0067_add_binary;
pub mod problem_0069_sqrtx;
pub mod problem_0070_climbing_stairs;
pub mod problem_0071_simplify_path;
pub mod problem_0072_edit_distance;
pub mod problem_0073_set_matrix_zeroes;
pub mod problem_0074_search_a_2d_matrix;
pub mod problem_0075_sort_colors;
pub mod problem_0077_combinations;
pub mod problem_0078_subsets;
pub mod problem_0079_word_search;
pub mod problem_0080_remove_duplicates_from_sorted_array_ii;
pub mod problem_0081_search_in_rotated_sorted_array_ii;
pub mod problem_0082_remove_duplicates_from_sorted_list_ii;
pub mod problem_0083_remove_duplicates_from_sorted_list;
pub mod problem_0086_partition_list;
pub mod problem_0088_merge_sorted_array;
pub mod problem_0089_gray_code;
pub mod problem_0090_subsets_ii;
pub mod problem_0091_decode_ways;
pub mod problem_0092_reverse_linked_list_ii;
pub mod problem_0093_restore_ip_addresses;
pub mod problem_0094_binary_tree_inorder_traversal;
pub mod problem_0095_unique_binary_search_trees_ii;
pub mod problem_0096_unique_binary_search_trees;
pub mod problem_0097_interleaving_string;
pub mod problem_0098_validate_binary_search_tree;
pub mod problem_0100_same_tree;
pub mod problem_0101_symmetric_tree;
pub mod problem_0102_binary_tree_level_order_traversal;
pub mod problem_0103_binary_tree_zigzag_level_order_traversal;
pub mod problem_0104_maximum_depth_of_binary_tree;
pub mod problem_0105_construct_binary_tree_from_preorder_and_inorder_traversal;
pub mod problem_0106_construct_binary_tree_from_inorder_and_postorder_traversal;
pub mod problem_0107_binary_tree_level_order_traversal_ii;
pub mod problem_0108_convert_sorted_array_to_binary_search_tree;
pub mod problem_0109_convert_sorted_list_to_binary_search_tree;
pub mod problem_0110_balanced_binary_tree;
pub mod problem_0111_minimum_depth_of_binary_tree;
pub mod problem_0112_path_sum;
pub mod problem_0113_path_sum_ii;
pub mod problem_0114_flatten_binary_tree_to_linked_list;
pub mod problem_0118_pascals_triangle;
pub mod problem_0119_pascals_triangle_ii;
pub mod problem_0120_triangle;
pub mod problem_0121_best_time_to_buy_and_sell_stock;
pub mod problem_0122_best_time_to_buy_and_sell_stock_ii;
pub mod problem_0125_valid_palindrome;
pub mod problem_0128_longest_consecutive_sequence;
pub mod problem_0129_sum_root_to_leaf_numbers;
pub mod problem_0131_palindrome_partitioning;
pub mod problem_0132_palindrome_partitioning_ii;
pub mod problem_0136_single_number;
pub mod problem_0139_word_break;
pub mod problem_0144_binary_tree_preorder_traversal;
pub mod problem_0145_binary_tree_postorder_traversal;
pub mod problem_0206_reverse_linked_list;

#[cfg(test)]
pub mod test_utilities;
