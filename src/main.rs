type Processor<T, Out> = fn (t: &T) -> Out;

fn parallel_process<T, Out>(in_args: &Vec<T>, processor: Processor<T, Out>, threshold: usize) -> Vec<Out> 
where 
    T: Send + Sync,
    Out: Send + Sync,
{
    let mut res: Vec<Out> = Vec::with_capacity(in_args.len());
    let n_threads = in_args.len()/threshold;

    let _ = crossbeam::scope(|scope|{
   
        let offset = in_args.len() % threshold;
        let mut handles = Vec::with_capacity(n_threads);

        for i in 0..n_threads {

            let begin = i *threshold + offset;
            let end = begin + threshold;

            println!("spawning a thread for [{}, {}]", begin, end);
            let sl = &in_args[begin..end];

            handles.push(
                scope.spawn(move |_| {
                    let mut res = Vec::with_capacity(sl.len());

                    for i in sl.iter() {
                        res.push(processor(i));
                    }
                    
                    res
                })
            );
        }

        println!("processing the rest [{}, {}]", 0, offset);
        for i in 0..offset {
            res.push(processor(&in_args[i]));
        }
    
        for h in handles {
            res.extend(h.join().unwrap());
        }
    });

    res
}

fn to_string(arg: &u32) -> String {
    format!("{}", arg)
}

fn main() {
    let mut in_args: Vec<u32> = Vec::new();
    in_args.resize_with(100, Default::default);
    for (i, elem) in in_args.iter_mut().enumerate(){
        *elem = i as u32;
    }

    let res = parallel_process(&in_args, to_string, 27);
    println!("{:?}", res);
}