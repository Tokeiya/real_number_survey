pub enum BitArray<const N: usize>
where
	[bool; N - 1]:,
{
	Normal([bool; N]),
	Subnormal([bool; N - 1]),
}
