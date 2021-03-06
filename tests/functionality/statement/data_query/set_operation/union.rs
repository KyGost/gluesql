crate::util_macros::testcase!(
	(|mut glue: multisql::Glue| {
		crate::util_macros::assert_select!(
			glue,
			r#"
				VALUES (
					'Test',
					1
				), (
					'Test2',
					2
				)
				UNION
				VALUES (
					'Test3',
					3
				)
			"# =>
				unnamed_0 = Str, unnamed_1 = I64:
				("Test", 1), ("Test2", 2), ("Test3", 3)
		);
		crate::util_macros::assert_select!(
			glue,
			r#"
				VALUES (
					'Test',
					1
				), (
					'Test2',
					2
				), (
					'Test3',
					3
				)
				UNION
				VALUES (
					'Test3',
					3
				)
			"# =>
			unnamed_0 = Str, unnamed_1= I64:
			("Test", 1), ("Test2", 2), ("Test3", 3)
		);
		crate::util_macros::assert_select!(
			glue,
			r#"
				VALUES (
					'Test',
					1
				), (
					'Test2',
					2
				), (
					'Test3',
					3
				)
				UNION ALL
				VALUES (
					'Test3',
					3
				)
			"# =>
			unnamed_0 = Str, unnamed_1 = I64:
			("Test", 1), ("Test2", 2), ("Test3", 3), ("Test3", 3)
		);
	})
);
