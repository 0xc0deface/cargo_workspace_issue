use sqlite::{OpenFlags, State};

fn funky()
{
	let connection = sqlite::open( "test.sqlite" ).unwrap();

	connection.execute( "
		CREATE TABLE IF NOT EXISTS test( t1 TEXT, t2 TEXT );
		INSERT INTO test(t1, t2) VALUES( 'x', 'y' );
		").unwrap();
}


#[cfg(test)]
mod tests
{
	use super::*;

	#[test]
	fn regex_test()
	{
		funky();
	}
}

