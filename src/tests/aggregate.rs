use crate::*;

test_case!(aggregate, async move {
	run!(
		"
        CREATE TABLE Item (
            id INTEGER,
            quantity INTEGER,
            age INTEGER NULL,
        );
    "
	);
	run!(
		"
        INSERT INTO Item (id, quantity, age) VALUES
            (1, 10,   11),
            (2,  0,   90),
            (3,  9, NULL),
            (4,  3,    3),
            (5, 25, NULL);
    "
	);

	use Value::*;

	let test_cases = vec![
		("SELECT COUNT(1) FROM Item", select!("COUNT(1)"; I64; 5)),
		("SELECT count(1) FROM Item", select!("count(1)"; I64; 5)),
		(
			"SELECT COUNT(1), COUNT(1) FROM Item",
			select!("COUNT(1)" | "COUNT(1)"; I64 | I64; 5 5),
		),
		(
			"SELECT SUM(quantity), MAX(quantity), MIN(quantity) FROM Item",
			select!(
				"SUM(quantity)" | "MAX(quantity)" | "MIN(quantity)"
				I64             | I64             | I64;
				47                25                0
			),
		),
		(
			"SELECT SUM(quantity + 1) FROM Item",
			select!(
				"SUM(quantity)"
				I64;
				52
			),
		),
		(
			"SELECT SUM(quantity) * 2 + MAX(quantity) - 3 / 1 FROM Item",
			select!("SUM(quantity) * 2 + MAX(quantity) - 3 / 1"; I64; 116),
		),
		(
			"SELECT SUM(age), MAX(age), MIN(age) FROM Item",
			select_with_null!(
				"SUM(age)" | "MAX(age)" | "MIN(age)";
				Null         I64(90)     I64(3)
			),
		),
		(
			"SELECT SUM(age) + SUM(quantity) FROM Item",
			select_with_null!("SUM(age) + SUM(quantity)"; Null),
		),
		(
			"SELECT COUNT(age), COUNT(quantity) FROM Item",
			select!("unnamed_0" | "unnamed_1"; I64 | I64; 3 5),
		),
		(
			"SELECT AVG(quantity) FROM Item",
			select!("unnamed_0"; I64; 9),
		),
		("SELECT SUM(1 + 2) FROM Item", select!("unnamed_0"; I64; 15)),
	];

	for (sql, expected) in test_cases.into_iter() {
		test!(Ok(expected), sql);
	}

	let error_cases = vec![
		(
			RecipeError::MissingColumn(vec![
				String::from("id"),
				String::from("name"),
				String::from("ok"),
			])
			.into(),
			"SELECT SUM(id.name.ok) FROM Item;",
		),
		(
			RecipeError::MissingColumn(vec![String::from("num")]).into(),
			"SELECT SUM(num) FROM Item;",
		),
	];

	for (error, sql) in error_cases.into_iter() {
		test!(Err(error), sql);
	}
});

test_case!(group_by, async move {
	run!(
		"
        CREATE TABLE Item (
            id INTEGER,
            quantity INTEGER NULL,
            city TEXT,
            ratio FLOAT,
        );
    "
	);
	run!(
		"
        INSERT INTO Item (id, quantity, city, ratio) VALUES
            (1,   10,   \"Seoul\",  0.2),
            (2,    0,   \"Dhaka\",  0.9),
            (3, NULL, \"Beijing\",  1.1),
            (3,   30, \"Daejeon\",  3.2),
            (4,   11,   \"Seoul\", 11.1),
            (5,   24, \"Seattle\", 6.11);
    "
	);

	use Value::*;

	let test_cases = vec![
		(
			"SELECT id, COUNT(1) FROM Item GROUP BY id",
			select!(id | "COUNT(1)"; I64 | I64; 1 1; 2 1; 3 2; 4 1; 5 1),
		),
		(
			"SELECT id FROM Item GROUP BY id",
			select!(id; I64; 1; 2; 3; 4; 5),
		),
		(
			"SELECT SUM(quantity), COUNT(1), city FROM Item GROUP BY city",
			select_with_null!(
				"SUM(quantity)" | "COUNT(1)" | city;
				Null              I64(1)       Str("Beijing".to_owned());
				I64(30)           I64(1)       Str("Daejeon".to_owned());
				I64(0)            I64(1)       Str("Dhaka".to_owned());
				I64(24)           I64(1)       Str("Seattle".to_owned());
				I64(21)           I64(2)       Str("Seoul".to_owned())
			),
		),
		(
			"SELECT id, city FROM Item GROUP BY city",
			select!(
				id  | city
				I64 | Str;
				3     "Beijing".to_owned();
				3     "Daejeon".to_owned();
				2     "Dhaka".to_owned();
				5     "Seattle".to_owned();
				1     "Seoul".to_owned()
			),
		),
		(
			"SELECT ratio FROM Item GROUP BY id, city",
			select!(ratio; F64; 0.2; 0.9; 1.1; 3.2; 11.1; 6.11),
		),
		(
			"SELECT ratio FROM Item GROUP BY id, city HAVING ratio > 10",
			select!(ratio; F64; 11.1),
		),
		(
			"SELECT SUM(quantity), COUNT(1), city FROM Item GROUP BY city HAVING COUNT(1) > 1",
			select!(
				"SUM(quantity)" | "COUNT(1)" | city
				I64          | I64        | Str;
				21             2            "Seoul".to_owned()
			),
		),
	];

	for (sql, expected) in test_cases.into_iter() {
		test!(Ok(expected), sql);
	}
});
