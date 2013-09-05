extern mod extra;
extern mod simplify;
use simplify::*;
use extra::json::*;
use extra::time::*;
use std::path;
use std::os;
use std::io::*;
use std::float;
fn dealList(l:~[Json])->~[Point]{
    println(fmt!("from %?",l.len()));
	l.map(|b|{
		match *b{
			List([Number(x),Number(y)])=>Point{x:x,y:y},
			_=>Point{x:0.0,y:0.0}
		}
	})
}
fn dealJson (s:~str)->~[Point]{
	match from_str(s){
		Ok(j)=> match j{
		    List(l)=>dealList(l),
		    _=>~[Point{x:0.0,y:0.0}]
		   },
	    _=>~[Point{x:0.0,y:0.0}]
	}
}
fn writeOut ( j:~[Point] , outPath:~path::Path) {
    println(fmt!("to %?",j.len()));
	match buffered_file_writer(outPath) {
	    Ok(writer)=>j.to_json().to_writer(writer),
	    Err(e)=>println(fmt!("%?",e))
	}
	true;
}
fn main() {
    let args : ~[~str] = os::args();
	let reader = read_whole_file_str(~path::Path(args[1]));
	let outPath = ~path::Path(args[2]);
	let simp = match float::from_str(args[3]){
	    Some(s)=>s,
	    _=>1.0f
	};
	match reader{
		Ok(points)=> {
		let p :~[Point] = dealJson(points);
		let startT :u64 = precise_time_ns();
		let out = simplify(p,simp,false);
		let endT : u64 =  precise_time_ns();
		println(fmt!("time %?",endT-startT));
		 writeOut(out,outPath)
		 }
		Err(e)=>println(fmt!("%?",e))
	}
}
