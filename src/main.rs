use std::{marker::PhantomPinned, pin::Pin, ptr::NonNull, time::Instant};

mod router;

/*pub struct Something {
    number: usize,
}

pub struct Data {
    something: Something,
    reference: NonNull<Something>,
    _pin: PhantomPinned,
}

impl<'data> Data {
    pub fn new() -> Pin<&'data Self> {
        let ret = Data { something: Something{number: 5}, reference: NonNull::dangling(), _pin: PhantomPinned};

        let mut boxed = Box::pin(ret);
        let reference = NonNull::from(&boxed.something);
        // we know this is safe because modifying a field doesn't move the whole struct
        unsafe {
            let mut_ref: Pin<&mut Self> = Pin::as_mut(&mut boxed);
            Pin::get_unchecked_mut(mut_ref).reference = reference;
        }

        boxed
    }

    pub fn increase_through_ref(mut self: Pin<&mut Self>) {
        let field = unsafe { &mut self.get_unchecked_mut().something };
    }
}*/

fn main() {
    //let data = Data::new();
    //data.increase_through_ref();

    let router = router::new_router("./data/germany-latest.osm.pbf");

    let (from_lat, from_lon) = (51.046527, 3.719028);
    let (to_lat, to_lon) = (51.028482, 3.639622);

    let now = Instant::now();
    let result = router.route((from_lat, from_lon), (to_lat, to_lon));

    if let Some(routing_result) = result {
        if !routing_result.paths.is_empty() {
            let nodes = router.get_wkt(&routing_result.paths[0]);
            println!("result: {} in {} ms", nodes, now.elapsed().as_millis());
        } else {
            println!("no path found")
        }
    } else {
        println!("no route found")
    }

    //TODO create api
}
