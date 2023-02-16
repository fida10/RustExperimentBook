mod chp_two_guessing_game;
mod chp_three_common_programming_concepts;
mod chp_three_ending_exercises;

fn main() {

    chp_two_guessing_game::guessing_game();

    chp_three_common_programming_concepts::question_two();
    chp_three_common_programming_concepts::loop_example_two();
    chp_three_common_programming_concepts::loop_example(10);
    chp_three_common_programming_concepts::another_function(23);
    chp_three_common_programming_concepts::printTwoParam(45, 7.5);
    let x: bool = chp_three_common_programming_concepts::returnSomething();
    println!("{}", x);
    println!("{}", chp_three_common_programming_concepts::returnSomething());
    println!("{}", chp_three_common_programming_concepts::returnNumber());
    println!("{}", chp_three_common_programming_concepts::returnNumberWithParameter(45));
    chp_three_common_programming_concepts::if_statement(25);
    println!("{}", chp_three_common_programming_concepts::if_statement_with_let());

    println!("{}", chp_three_ending_exercises::convert_f_to_c(46.0));
    println!("{}", chp_three_ending_exercises::convert_c_to_f(29.0));

    println!("{}", chp_three_ending_exercises::fibonacci_series(0));
}