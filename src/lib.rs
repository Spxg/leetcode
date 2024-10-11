#![warn(
    absolute_paths_not_starting_with_crate,
    explicit_outlives_requirements,
    let_underscore_drop,
    macro_use_extern_crate,
    meta_variable_misuse,
    missing_abi,
    missing_docs,
    noop_method_call,
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
pub mod problem_0068_text_justification;
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
pub mod problem_0461_hamming_distance;
pub mod problem_0462_minimum_moves_to_equal_array_elements_ii;
pub mod problem_0468_validate_ip_address;
pub mod problem_0476_number_complement;
pub mod problem_0481_magical_string;
pub mod problem_0482_license_key_formatting;
pub mod problem_0485_max_consecutive_ones;
pub mod problem_0495_teemo_attacking;
pub mod problem_0498_diagonal_traverse;
pub mod problem_0500_keyboard_row;
pub mod problem_0502_ipo;
pub mod problem_0503_next_greater_element_ii;
pub mod problem_0504_base_7;
pub mod problem_0507_perfect_number;
pub mod problem_0509_fibonacci_number;
pub mod problem_0513_find_bottom_left_tree_value;
pub mod problem_0515_find_largest_value_in_each_tree_row;
pub mod problem_0520_detect_capital;
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
pub mod problem_0594_longest_harmonious_subsequence;
pub mod problem_0598_range_addition_ii;
pub mod problem_0599_minimum_index_sum_of_two_lists;
pub mod problem_0605_can_place_flowers;
pub mod problem_0606_construct_string_from_binary_tree;
pub mod problem_0623_add_one_row_to_tree;
pub mod problem_0628_maximum_product_of_three_numbers;
pub mod problem_0637_average_of_levels_in_binary_tree;
pub mod problem_0640_solve_the_equation;
pub mod problem_0643_maximum_average_subarray_i;
pub mod problem_0645_set_mismatch;
pub mod problem_0646_maximum_length_of_pair_chain;
pub mod problem_0647_palindromic_substrings;
pub mod problem_0648_replace_words;
pub mod problem_0650_2_keys_keyboard;
pub mod problem_0653_two_sum_iv_input_is_a_bst;
pub mod problem_0654_maximum_binary_tree;
pub mod problem_0657_robot_return_to_origin;
pub mod problem_0665_non_decreasing_array;
pub mod problem_0670_maximum_swap;
pub mod problem_0673_number_of_longest_increasing_subsequence;
pub mod problem_0674_longest_continuous_increasing_subsequence;
pub mod problem_0677_map_sum_pairs;
pub mod problem_0682_baseball_game;
pub mod problem_0693_binary_number_with_alternating_bits;
pub mod problem_0697_degree_of_an_array;
pub mod problem_0700_search_in_a_binary_search_tree;
pub mod problem_0704_binary_search;
pub mod problem_0709_to_lower_case;
pub mod problem_0713_subarray_product_less_than_k;
pub mod problem_0717_1_bit_and_2_bit_characters;
pub mod problem_0720_longest_word_in_dictionary;
pub mod problem_0724_find_pivot_index;
pub mod problem_0725_split_linked_list_in_parts;
pub mod problem_0728_self_dividing_numbers;
pub mod problem_0738_monotone_increasing_digits;
pub mod problem_0739_daily_temperatures;
pub mod problem_0744_find_smallest_letter_greater_than_target;
pub mod problem_0748_shortest_completing_word;
pub mod problem_0763_partition_labels;
pub mod problem_0769_max_chunks_to_make_sorted;
pub mod problem_0771_jewels_and_stones;
pub mod problem_0779_k_th_symbol_in_grammar;
pub mod problem_0784_letter_case_permutation;
pub mod problem_0791_custom_sort_string;
pub mod problem_0795_number_of_subarrays_with_bounded_maximum;
pub mod problem_0804_unique_morse_code_words;
pub mod problem_0806_number_of_lines_to_write_string;
pub mod problem_0809_expressive_words;
pub mod problem_0811_subdomain_visit_count;
pub mod problem_0814_binary_tree_pruning;
pub mod problem_0817_linked_list_components;
pub mod problem_0819_most_common_word;
pub mod problem_0821_shortest_distance_to_a_character;
pub mod problem_0825_friends_of_appropriate_ages;
pub mod problem_0826_most_profit_assigning_work;
pub mod problem_0831_masking_personal_information;
pub mod problem_0833_find_and_replace_in_string;
pub mod problem_0841_keys_and_rooms;
pub mod problem_0844_backspace_string_compare;
pub mod problem_0845_longest_mountain_in_array;
pub mod problem_0846_hand_of_straights;
pub mod problem_0851_loud_and_rich;
pub mod problem_0856_score_of_parentheses;
pub mod problem_0859_buddy_strings;
pub mod problem_0868_binary_gap;
pub mod problem_0869_reordered_power_of_2;
pub mod problem_0870_advantage_shuffle;
pub mod problem_0872_leaf_similar_trees;
pub mod problem_0876_middle_of_the_linked_list;
pub mod problem_0881_boats_to_save_people;
pub mod problem_0884_uncommon_words_from_two_sentences;
pub mod problem_0889_construct_binary_tree_from_preorder_and_postorder_traversal;
pub mod problem_0890_find_and_replace_pattern;
pub mod problem_0895_maximum_frequency_stack;
pub mod problem_0900_rle_iterator;
pub mod problem_0915_partition_array_into_disjoint_intervals;
pub mod problem_0916_word_subsets;
pub mod problem_0917_reverse_only_letters;
pub mod problem_0919_complete_binary_tree_inserter;
pub mod problem_0921_minimum_add_to_make_parentheses_valid;
pub mod problem_0925_long_pressed_name;
pub mod problem_0929_unique_email_addresses;
pub mod problem_0930_binary_subarrays_with_sum;
pub mod problem_0931_minimum_falling_path_sum;
pub mod problem_0932_beautiful_array;
pub mod problem_0937_reorder_data_in_log_files;
pub mod problem_0938_range_sum_of_bst;
pub mod problem_0944_delete_columns_to_make_sorted;
pub mod problem_0945_minimum_increment_to_make_array_unique;
pub mod problem_0946_validate_stack_sequences;
pub mod problem_0947_most_stones_removed_with_same_row_or_column;
pub mod problem_0948_bag_of_tokens;
pub mod problem_0950_reveal_cards_in_increasing_order;
pub mod problem_0951_flip_equivalent_binary_trees;
pub mod problem_0958_check_completeness_of_a_binary_tree;
pub mod problem_0961_n_repeated_element_in_size_2n_array;
pub mod problem_0965_univalued_binary_tree;
pub mod problem_0967_numbers_with_same_consecutive_differences;
pub mod problem_0969_pancake_sorting;
pub mod problem_0971_flip_binary_tree_to_match_preorder_traversal;
pub mod problem_0973_k_closest_points_to_origin;
pub mod problem_0974_subarray_sums_divisible_by_k;
pub mod problem_0977_squares_of_a_sorted_array;
pub mod problem_0981_time_based_key_value_store;
pub mod problem_0984_string_without_aaa_or_bbb;
pub mod problem_0985_sum_of_even_numbers_after_queries;
pub mod problem_0986_interval_list_intersections;
pub mod problem_0988_smallest_string_starting_from_leaf;
pub mod problem_0989_add_to_array_form_of_integer;
pub mod problem_0994_rotting_oranges;
pub mod problem_1002_find_common_characters;
pub mod problem_1006_clumsy_factorial;
pub mod problem_1008_construct_binary_search_tree_from_preorder_traversal;
pub mod problem_1009_complement_of_base_10_integer;
pub mod problem_1010_pairs_of_songs_with_total_durations_divisible_by_60;
pub mod problem_1018_binary_prefix_divisible_by_5;
pub mod problem_1023_camelcase_matching;
pub mod problem_1037_valid_boomerang;
pub mod problem_1038_binary_search_tree_to_greater_sum_tree;
pub mod problem_1043_partition_array_for_maximum_sum;
pub mod problem_1046_last_stone_weight;
pub mod problem_1051_height_checker;
pub mod problem_1054_distant_barcodes;
pub mod problem_1061_lexicographically_smallest_equivalent_string;
pub mod problem_1089_duplicate_zeros;
pub mod problem_1090_largest_values_from_labels;
pub mod problem_1093_statistics_from_a_large_sample;
pub mod problem_1094_car_pooling;
pub mod problem_1104_path_in_zigzag_labelled_binary_tree;
pub mod problem_1105_filling_bookcase_shelves;
pub mod problem_1122_relative_sort_array;
pub mod problem_1146_snapshot_array;
pub mod problem_1161_maximum_level_sum_of_a_binary_tree;
pub mod problem_1170_compare_strings_by_frequency_of_the_smallest_character;
pub mod problem_1171_remove_zero_sum_consecutive_nodes_from_linked_list;
pub mod problem_1189_maximum_number_of_balloons;
pub mod problem_1190_reverse_substrings_between_each_pair_of_parentheses;
pub mod problem_1209_remove_all_adjacent_duplicates_in_string_ii;
pub mod problem_1218_longest_arithmetic_subsequence_of_given_difference;
pub mod problem_1221_split_a_string_in_balanced_strings;
pub mod problem_1233_remove_sub_folders_from_the_filesystem;
pub mod problem_1248_count_number_of_nice_subarrays;
pub mod problem_1261_find_elements_in_a_contaminated_binary_tree;
pub mod problem_1268_search_suggestions_system;
pub mod problem_1282_group_the_people_given_the_group_size_they_belong_to;
pub mod problem_1288_remove_covered_intervals;
pub mod problem_1291_sequential_digits;
pub mod problem_1302_deepest_leaves_sum;
pub mod problem_1310_xor_queries_of_a_subarray;
pub mod problem_2348_number_of_zero_filled_subarrays;

#[cfg(test)]
pub mod test_utilities;
