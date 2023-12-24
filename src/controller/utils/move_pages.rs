#[allow(unused_assignments)]
pub fn back_page(mut init: usize, mut end: usize, lines: usize) -> (usize, usize) {
    if init > 0{
        end = init;
        init -= lines;
    }else if init > 0 && init < lines{
        end = init;
        init += init-0;
    }else{
        init = 0;
        end = lines;
    }

    (init, end)
}

pub fn next_page(mut init: usize, mut end: usize, lines: usize,  size_max: &usize) -> (usize, usize) {
    if end <= size_max-lines{
        init = end;
        end += lines;
    }else if end < *size_max && end > size_max-lines{
        init = end;
        end += size_max-end;
    }

    (init, end)
}