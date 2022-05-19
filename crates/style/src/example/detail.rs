use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use unit_test::Bencher;

#[bench]
fn parse_number_bench(bench: &mut Bencher) {
  bench.iter(|| {
    let is_number = |char: char| -> bool { char.to_string().parse::<i32>().is_ok() };
    let list = vec!['a', '1'];
    list.iter().for_each(|x| {
      is_number(*x);
    });
  });
}

#[bench]
fn parse_number_compare_bench(bench: &mut Bencher) {
  bench.iter(|| {
    let is_number = |char: char| -> bool {
      vec!['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'].contains(&char)
    };
    let list = vec!['a', '1'];
    list.iter().for_each(|x| {
      is_number(*x);
    });
  });
}

#[bench]
fn parse_str_u32_bench(bench: &mut Bencher) {
  bench.iter(|| {
    let char = "0123456789";
    let charlist = char.chars().collect::<Vec<char>>();
    for item in charlist {
      println!("{:#?}", item);
    }
  });
}

#[bench]
fn parse_str_bench(bench: &mut Bencher) {
  bench.iter(|| {
    let chars = "0123456789";
    let charlist = chars
      .chars()
      .map(|x| String::from(x))
      .collect::<Vec<String>>();
    for item in charlist {
      println!("{:#?}", item);
    }
  });
}

#[bench]
fn scan_oirgin_bench(bench: &mut Bencher) {
  let mut txt = "a".to_string();
  let mut index = 0;
  while index < 1000000 {
    txt += "a";
    index += 1;
  }

  let list = txt.chars().map(|x| x).collect::<Vec<char>>();

  bench.iter(|| {
    let mut index = 0;
    let res = Rc::new(RefCell::new(vec![]));
    while index < list.len() {
      res.deref().borrow_mut().push(list.get(index).unwrap());
      index += 1;
    }
  });
}

fn traversal(
  arg_start: Option<usize>,
  charlist: &[char],
  exec: &mut dyn FnMut((&usize, &mut Vec<char>), &char) -> Result<(), String>,
) -> Result<(Vec<char>, usize), String> {
  let mut index = arg_start.unwrap_or(0);
  let mut temp: Vec<char> = vec![];
  while index < charlist.len() {
    let char = charlist.get(index).unwrap();
    let arg = (&index, &mut temp);
    let _res = exec(arg, char)?;
    index += 1;
  }
  Ok((vec![], index))
}

#[bench]
fn scan_bench(bench: &mut Bencher) {
  let mut txt = "a".to_string();
  let mut index = 0;
  while index < 1000000 {
    txt += "a";
    index += 1;
  }
  let list = txt.chars().map(|x| x).collect::<Vec<char>>();
  bench.iter(|| {
    let res = traversal(
      None,
      &list,
      &mut (|arg, charword| {
        let (_, temp) = arg;
        temp.push(*charword);
        Ok(())
      }),
    );
    match res {
      Ok(_) => 2,
      Err(_) => 1,
    };
  });
}
