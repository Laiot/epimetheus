const MetricName: &str = "__name__";
const AlertName: &str = "alertname";
const BucketLabel: &str = "le";
const InstanceName: &str = "instance";
const labelSep: u8 = b'\xfe';

static mut seps: Vec<u8> = vec![b'\xff'];

struct Label{
    name: &'static str,
    value: &'static str
}

struct Labels{
    values: Vec<Label>
}

impl Labels{
    fn len(self) -> usize{
        return self.values.len();
    }

    fn swap(&mut self, i: usize, j: usize){
        self.values.swap(i, j);
    }

    fn less(self, i: usize, j: usize) -> bool{
        return self.values[i].name < self.values[j].name;
    }

    fn string(self) -> String{
        let mut buf = String::new();
        
        buf.push('{');
        for (i, l) in self.values.iter().enumerate(){
            if i > 0{
                buf.push_str(", ");
            }
            buf.push_str(l.name);
            buf.push('=');
            buf.push('\"');
            buf.push_str(l.value);
            buf.push('\"');
        }
        buf.push('}');

        return buf;
    }

    fn bytes(self, buf: Vec<u8>){
        
    }
}
