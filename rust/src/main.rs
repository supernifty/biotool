extern crate bio;
use std::io;
use std::cmp;
use bio::io::fasta;
use std::fmt;

struct FastaStats {
   min_len: u64,
   average_len: u64,
   max_len: u64,
   total: u64,
   num_seqs: u64,
}

impl FastaStats {
   pub fn new<R: io::Read>(reader: R) -> Option<FastaStats> {
      let fasta_reader = fasta::Reader::new(reader);
      let mut num_seqs:u64 = 0;
      let mut total:u64 = 0;
      let mut max_len:u64 = 0;
      let mut min_len:u64 = 0;
      let mut this_len:u64;

      for next in fasta_reader.records() {
         match next { 
            Ok(record) => {
               num_seqs += 1;
               this_len = record.seq().len() as u64;
               total += this_len; 
               if num_seqs == 1 {
                  max_len = this_len;
                  min_len = this_len;
               }
               else {
                  max_len = cmp::max(max_len, this_len);
                  min_len = cmp::min(min_len, this_len);
               }
            }, 
            // XXX handle errors properly
            Err(error) => println!("{}", error),
         }
      }
      if num_seqs > 0 {
         // XXX check whether integer division does floor
         let average_len = ((total as f64) / (num_seqs as f64)).floor() as u64;
         Some(FastaStats { min_len: min_len,
                           average_len: average_len, 
                           max_len: max_len, 
                           total: total, 
                           num_seqs: num_seqs })
      } 
      else {
         None
      }
   }
}

impl fmt::Display for FastaStats {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{}\t{}\t{}\t{}\t{}", self.num_seqs, self.total, self.min_len, self.average_len, self.max_len)
   }
}


fn main() {
   println!("FILENAME\tTOTAL\tNUMSEQ\tMIN\tAVG\tMAX");
   match FastaStats::new(io::stdin()) {
      Some(stats) => {
         println!("{}", stats);
      }
      None => {
         println!("0\t0\t-\t-\t-");
      }
   }
}