fn main(){
	let t:f64 = 450000.0;
	let m:f64 = 1500000.0;
	let h:f64 = 750000.0;
	let d:f64 = 2850000.0;
	let a:f64 = 250000.0;

	//totals
	let tt = t * 2.0;
	println!("TOSHIBA total sales is {}",tt);
	let mt = m * 1.0;
	println!("MAC total sales is {}",mt);
	let ht = h * 3.0;
	println!("HP total sales is {}",ht);
	let dt = d * 3.0;
	println!("DELL total sales is {}",dt);
	let at = a * 1.0;
	println!("ACER total sales is {}",at);

	//sum of totals
	let sum = tt + mt + ht + dt + at;
	println!("TOTAL SALES is {}",sum);

	//sum of sales
	let units = 2.0 + 1.0 + 3.0 + 3.0 + 1.0;
	println!("UNITS SOLD TOTALLY is {}",units);

	//average
	let average = sum / units;
	println!("THE AVERAGE IS {}",average);

}