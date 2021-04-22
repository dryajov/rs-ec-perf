use color_eyre::Result;
use reed_solomon_benches::WrappedShard;

fn main() -> Result<()> {
	color_eyre::install()?;

	#[allow(unused)]
	use reed_solomon_benches::{BYTES, N_SHARDS};

	{
		use reed_solomon_benches::novelpoly;
		reed_solomon_tester::roundtrip(
			novelpoly::encode::<WrappedShard>,
			novelpoly::reconstruct::<WrappedShard>,
			&BYTES[..],
			N_SHARDS,
		)?;
	}

	#[cfg(feature = "novelpoly-with-alt-cxx-impl")]
	{
		use reed_solomon_benches::novelpoly;
		reed_solomon_tester::roundtrip(novelpoly::cxx::encode, novelpoly::cxx::reconstruct, &BYTES[..], N_SHARDS)?;
	}

	#[cfg(feature = "naive")]
	{
		use reed_solomon_benches::naive;
		reed_solomon_tester::roundtrip(
			naive::encode::<WrappedShard>,
			naive::reconstruct::<WrappedShard>,
			&BYTES[..],
			N_SHARDS,
		)?;
	}

	Ok(())
}
