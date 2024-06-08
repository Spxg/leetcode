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
    dead_code,
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
pub mod problem_0029_divide_two_integers;
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
pub mod problem_0143_reorder_list;
pub mod problem_0144_binary_tree_preorder_traversal;
pub mod problem_0145_binary_tree_postorder_traversal;
pub mod problem_0150_evaluate_reverse_polish_notation;
pub mod problem_0151_reverse_words_in_a_string;
pub mod problem_0152_maximum_product_subarray;
pub mod problem_0153_find_minimum_in_rotated_sorted_array;
pub mod problem_0154_find_minimum_in_rotated_sorted_array_ii;
pub mod problem_0155_min_stack;
pub mod problem_0162_find_peak_element;
pub mod problem_0164_maximum_gap;
pub mod problem_0165_compare_version_numbers;
pub mod problem_0166_fraction_to_recurring_decimal;
pub mod problem_0167_two_sum_ii_input_array_is_sorted;
pub mod problem_0168_excel_sheet_column_title;
pub mod problem_0169_majority_element;
pub mod problem_0171_excel_sheet_column_number;
pub mod problem_0173_binary_search_tree_iterator;
pub mod problem_0179_largest_number;
pub mod problem_0189_rotate_array;
pub mod problem_0190_reverse_bits;
pub mod problem_0191_number_of_1_bits;
pub mod problem_0198_house_robber;
pub mod problem_0199_binary_tree_right_side_view;
pub mod problem_0201_bitwise_and_of_numbers_range;
pub mod problem_0202_happy_number;
pub mod problem_0203_remove_linked_list_elements;
pub mod problem_0205_isomorphic_strings;
pub mod problem_0206_reverse_linked_list;
pub mod problem_0207_course_schedule;
pub mod problem_0208_implement_trie_prefix_tree;
pub mod problem_0209_minimum_size_subarray_sum;
pub mod problem_0210_course_schedule_ii;
pub mod problem_0211_design_add_and_search_words_data_structure;
pub mod problem_0213_house_robber_ii;
pub mod problem_0216_combination_sum_iii;
pub mod problem_0217_contains_duplicate;
pub mod problem_0219_contains_duplicate_ii;
pub mod problem_0225_implement_stack_using_queues;
pub mod problem_0226_invert_binary_tree;
pub mod problem_0227_basic_calculator_ii;
pub mod problem_0228_summary_ranges;
pub mod problem_0230_kth_smallest_element_in_a_bst;
pub mod problem_0231_power_of_two;
pub mod problem_0232_implement_queue_using_stacks;
pub mod problem_0234_palindrome_linked_list;
pub mod problem_0235_lowest_common_ancestor_of_a_binary_search_tree;
pub mod problem_0240_search_a_2d_matrix_ii;
pub mod problem_0242_valid_anagram;
pub mod problem_0257_binary_tree_paths;
pub mod problem_0258_add_digits;
pub mod problem_0260_single_number_iii;
pub mod problem_0263_ugly_number;
pub mod problem_0264_ugly_number_ii;
pub mod problem_0268_missing_number;
pub mod problem_0274_h_index;
pub mod problem_0275_h_index_ii;
pub mod problem_0278_first_bad_version;
pub mod problem_0283_move_zeroes;
pub mod problem_0290_word_pattern;
pub mod problem_0299_bulls_and_cows;
pub mod problem_0303_range_sum_query_immutable;
pub mod problem_0304_range_sum_query_2d_immutable;
pub mod problem_0307_range_sum_query_mutable;
pub mod problem_0316_remove_duplicate_letters;
pub mod problem_0322_coin_change;
pub mod problem_0326_power_of_three;
pub mod problem_0328_odd_even_linked_list;
pub mod problem_0331_verify_preorder_serialization_of_a_binary_tree;
pub mod problem_0338_counting_bits;
pub mod problem_0341_flatten_nested_list_iterator;
pub mod problem_0342_power_of_four;
pub mod problem_0343_integer_break;
pub mod problem_0344_reverse_string;
pub mod problem_0345_reverse_vowels_of_a_string;
pub mod problem_0349_intersection_of_two_arrays;
pub mod problem_0355_design_twitter;
pub mod problem_0367_valid_perfect_square;
pub mod problem_0374_guess_number_higher_or_lower;
pub mod problem_0383_ransom_note;
pub mod problem_0386_lexicographical_numbers;
pub mod problem_0387_first_unique_character_in_a_string;
pub mod problem_0389_find_the_difference;
pub mod problem_0390_elimination_game;
pub mod problem_0392_is_subsequence;
pub mod problem_0393_utf_8_validation;
pub mod problem_0394_decode_string;
pub mod problem_0396_rotate_function;
pub mod problem_0397_integer_replacement;
pub mod problem_0398_random_pick_index;
pub mod problem_0400_nth_digit;
pub mod problem_0404_sum_of_left_leaves;
pub mod problem_0405_convert_a_number_to_hexadecimal;
pub mod problem_0409_longest_palindrome;
pub mod problem_0412_fizz_buzz;
pub mod problem_0413_arithmetic_slices;
pub mod problem_0415_add_strings;
pub mod problem_0416_partition_equal_subset_sum;
pub mod problem_0423_reconstruct_original_digits_from_english;
pub mod problem_0434_number_of_segments_in_a_string;
pub mod problem_0437_path_sum_iii;
pub mod problem_0438_find_all_anagrams_in_a_string;
pub mod problem_0441_arranging_coins;
pub mod problem_0442_find_all_duplicates_in_an_array;
pub mod problem_0443_string_compression;
pub mod problem_0448_find_all_numbers_disappeared_in_an_array;
pub mod problem_0451_sort_characters_by_frequency;
pub mod problem_0453_minimum_moves_to_equal_array_elements;
pub mod problem_0455_assign_cookies;
pub mod problem_0462_minimum_moves_to_equal_array_elements_ii;
pub mod problem_0468_validate_ip_address;
pub mod problem_0476_number_complement;
pub mod problem_0481_magical_string;
pub mod problem_0482_license_key_formatting;
pub mod problem_0485_max_consecutive_ones;
pub mod problem_0495_teemo_attacking;
pub mod problem_0498_diagonal_traverse;
pub mod problem_0503_next_greater_element_ii;
pub mod problem_0507_perfect_number;
pub mod problem_0513_find_bottom_left_tree_value;
pub mod problem_0515_find_largest_value_in_each_tree_row;
pub mod problem_0523_continuous_subarray_sum;
pub mod problem_0525_contiguous_array;
pub mod problem_0530_minimum_absolute_difference_in_bst;
pub mod problem_0532_k_diff_pairs_in_an_array;
pub mod problem_0537_complex_number_multiplication;
pub mod problem_0538_convert_bst_to_greater_tree;
pub mod problem_0539_minimum_time_difference;
pub mod problem_0540_single_element_in_a_sorted_array;
pub mod problem_0541_reverse_string_ii;
pub mod problem_0547_number_of_provinces;
pub mod problem_0551_student_attendance_record_i;
pub mod problem_0556_next_greater_element_iii;
pub mod problem_0557_reverse_words_in_a_string_iii;
pub mod problem_0560_subarray_sum_equals_k;
pub mod problem_0561_array_partition;
pub mod problem_0567_permutation_in_string;
pub mod problem_0581_shortest_unsorted_continuous_subarray;
pub mod problem_0606_construct_string_from_binary_tree;
pub mod problem_0623_add_one_row_to_tree;
pub mod problem_0643_maximum_average_subarray_i;
pub mod problem_0646_maximum_length_of_pair_chain;
pub mod problem_0647_palindromic_substrings;
pub mod problem_0648_replace_words;
pub mod problem_0650_2_keys_keyboard;
pub mod problem_0653_two_sum_iv_input_is_a_bst;
pub mod problem_0654_maximum_binary_tree;
pub mod problem_0665_non_decreasing_array;
pub mod problem_0670_maximum_swap;
pub mod problem_0673_number_of_longest_increasing_subsequence;
pub mod problem_0674_longest_continuous_increasing_subsequence;
pub mod problem_0677_map_sum_pairs;
pub mod problem_0709_to_lower_case;
pub mod problem_0717_1_bit_and_2_bit_characters;
pub mod problem_0720_longest_word_in_dictionary;
pub mod problem_0738_monotone_increasing_digits;
pub mod problem_0739_daily_temperatures;
pub mod problem_0769_max_chunks_to_make_sorted;
pub mod problem_0779_k_th_symbol_in_grammar;
pub mod problem_0784_letter_case_permutation;
pub mod problem_0791_custom_sort_string;
pub mod problem_0795_number_of_subarrays_with_bounded_maximum;
pub mod problem_0811_subdomain_visit_count;
pub mod problem_0814_binary_tree_pruning;
pub mod problem_0821_shortest_distance_to_a_character;
pub mod problem_0831_masking_personal_information;
pub mod problem_0844_backspace_string_compare;
pub mod problem_0845_longest_mountain_in_array;
pub mod problem_0859_buddy_strings;
pub mod problem_0917_reverse_only_letters;
pub mod problem_2348_number_of_zero_filled_subarrays;

#[cfg(test)]
pub mod test_utilities;
