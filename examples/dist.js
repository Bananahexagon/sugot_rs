"use strict"

    const $sugot_println = console.log;
function $sugot_main () {let $sugot_i = 0;while (($sugot_i !== 100)) {$sugot_fizz_buzz($sugot_i, );$sugot_i = ($sugot_i + 1);};}function $sugot_fizz_buzz ($sugot_i,) {if ((($sugot_i % 15) === 0)) {$sugot_println("FizzBuzz", );} else {if ((($sugot_i % 3) === 0)) {$sugot_println("Fizz", );} else {if ((($sugot_i % 5) === 0)) {$sugot_println("Buzz", );} else {$sugot_println($sugot_i, );};};};}$sugot_main()