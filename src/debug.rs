use std::fmt;

use forest::Forest;
use recognizer::Recognizer;

impl<'g, F: Forest> fmt::Debug for Recognizer<'g, F> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f,
            "Recognizer {{ grammar: {:?}, \
            predicted: {:?}, medial: {:?}, \
            complete: {:?}, indices: {:?}, \
            current_medial_start: {:?}, earleme: {:?} }}",
			self.grammar,
			&self.predicted,
			&self.medial,
			&self.complete,
			&self.indices,
			&self.current_medial_start,
			&self.earleme
		)
	}
}
