verus! {

fn test_function(x: u32) -> (res : u32)
  requires x < 100
  ensures res < 200
{
    x * 2
}

} // verus!
