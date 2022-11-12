fn main() {

  /// 基本形の演算
  println!(" ***** ***** ***** ***** ***** ");
  println!(" ***** ***** 基本形 ***** ***** ");
  println!(" ***** ***** ***** ***** ***** ");
  {
    println!("1 + 2 = {}", 1 + 2);
    println!("3 - 2 = {}", 3 - 2);
    println!("4 * 3 = {}", 4 * 3);
    println!("8 / 3 = {}", 8 / 3);
    println!("7 % 5 = {}", 7 % 5);
  }

  /// データ型
  {
    println!("8 / 3 = {}", 8 / 3);
    println!("8 as f32 / 3 as f32 = {}", 8 as f32 / 3 as f32);
    println!("8 as f64 / 3 as f64 = {}", 8 as f64 / 3 as f64);
  }

  /// オーバーフロー
  {
    // println!("{}", 255 as u8 + 1); // 【【エラーが発生します】】

    /// オーバフローへの対策
    /// [1]
    /// wrapping_***メソッドは溢れた桁を無視します。
    println!("wrapping -> {}", (255 as u8).wrapping_add(1));
    /// [2]
    /// checked_***メソッドは成功した場合にT型、溢れた場合にNoneを返すMaybe型のデータを返します。
    /// Maybe型のデータから値を取り出すにはunwrapメソッドを使用します。
    /// Noneが入っている場合にはパニックが発生します。
    // println!("checked -> {}", (255 as u8).checked_add(1).unwrap()); // 【【エラーが発生します】】
    /// [3]
    /// overflowing_***メソッドは演算結果と成功したかの真偽値をタプル型で返します。
    /// Goっぽいですね♪
    /// オーバフローしたら「true」、オーバフローしなければ「false」となることに注意して下さい。
    let (result, successed) = (255 as u8).overflowing_add(1);
    println!("result -> {}", result);
    println!("successed -> {}", successed);
  }

  /// アンダーフロー
  {
    println!("10.0 as f32 / 3.0 as f32 = {}", 10.0 as f32 / 3.0 as f32);
    println!("10.0 as f64 / 3.0 as f64 = {}", 10.0 as f64 / 3.0 as f64);
  }
}
