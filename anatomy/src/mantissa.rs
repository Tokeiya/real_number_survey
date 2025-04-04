use std::fmt::Debug;
use std::ops::Index;

/// N ビットの固定精度仮数域を表すトレイト。
///
/// # ジェネリックパラメータ
/// * `N` - 仮数域のビット長
///
/// # 制約
/// * `Self` は `Index<usize, Output = bool>` を実装する必要があります
///
/// # 関連型
/// * `Underlying` - 内部表現の型。`Debug` と `Copy` を実装する必要があります
pub trait Mantissa<const N: usize>
where
	Self: Index<usize, Output = bool>,
{
	/// 仮数域の内部表現の型
	type Underlying: Debug + Copy;

	/// 仮数域の有効精度（先頭の1から末尾までのビット数）を返します
	fn effective_precision(&self) -> usize;

	/// 仮数域の内部表現を返します
	fn underlying(&self) -> Self::Underlying;

	/// 仮数域をブール値の配列として返します
	fn to_array(&self) -> [bool; N];
}
