//file to hold a matrix object
//By: David Hauck
//Date: 5/1/13

struct Matrix
{
	values : ~[~[int]],
	height : uint,
	width : uint
}

impl Matrix
{
	//creates a new Matrix
	pub fn new(height : uint, width : uint, values : ~[~[int]]) -> Matrix
	{
		Matrix{height : height, width : width, values : values}
	}

	//gets the object at a position in the Matrix
	pub fn get(&self, i: int, j : int) -> int
	{
		return self.values[i][j];
	}

	//displays the matrix on the screen
	pub fn display(&self)
	{
		let mut i = 0;
		let mut j = 0;
		for self.height.times
		{
			j = 0;
			for self.width.times
			{
				print(fmt!("%?\t", self.values[i][j]));
				j += 1;
			}
			i += 1;
			print("\r\n");
		}
	}

	//determines whether two matrices can be added together
	pub fn canAdd (&self, other : &Matrix) -> bool
	{
		(self.height == other.height && self.width == other.width)
	}

	//determines whether two matrices can be multiplied together
	pub fn canMult (&self, other : &Matrix) -> bool
	{
		(self.width == other.height)
	}

	//multiplies a matrix by a constant
	pub fn mulConst(&self, value : int) -> Matrix
	{
		let mut newValues = ~[ ~[ 0, ..0], ..0];
		let mut i = 0;
		let mut j = 0;
		for self.height.times
		{
			j = 0;
			newValues.push(~[ 0, ..0]);
			for self.width.times
			{
				newValues[i].push(self.values[i][j] * value);
				j += 1;
			}
			i += 1;
		}
		Matrix{height : self.height, width : self.width, values : newValues}
	}

}

// overrides the + operator for two matrices
impl Add<Matrix, Matrix> for Matrix
{

	pub fn add(&self, other : &Matrix) -> Matrix
	{
		let mut newValues = ~[ ~[0, ..0], ..0];
		let mut i = 0;
		let mut j = 0;
		
		for self.height.times
		{
			j = 0;
			newValues.push( ~[0, ..0]);
			for self.width.times
			{
				newValues[i].push(self.values[i][j] + other.values[i][j]);
				j += 1;
			}
			i += 1;
		}
		return Matrix{ height : self.height, width : self.width, values : newValues};
	}
}

//overrides the - operator for two matrices
impl Sub<Matrix, Matrix> for Matrix
{

	pub fn sub(&self, other : &Matrix) -> Matrix
	{
		let mut newValues = ~[ ~[0, ..0], ..0];
		let mut i = 0;
		let mut j = 0;
		
		for self.height.times
		{
			j = 0;
			newValues.push( ~[0, ..0]);
			for self.width.times
			{
				newValues[i].push(self.values[i][j] - other.values[i][j]);
				j += 1;
			}
			i += 1;
		}
		return Matrix{ height : self.height, width : self.width, values : newValues};
	}
}

//overrides the * operator for two matrices
impl Mul<Matrix, Matrix> for Matrix
{
	pub fn mul(&self, other : &Matrix) -> Matrix
	{
		let newHeight = self.height;
		let newWidth = other.width;
		let mut newArray = ~[ ~[0, ..0], ..0];
		let mut i = 0;
		let mut j = 0;
		for newHeight.times
		{
			j = 0;
			newArray.push(~[0, ..0]);
			for newWidth.times
			{
				let mut total = 0;
				let mut k = 0;
				for self.width.times
				{
					total += self.values[i][k] * other.values[k][j];
					k += 1;
				}
				newArray[i].push(total);
				j += 1;
			}
			i += 1;
		}
		Matrix{height: newHeight, width : newWidth, values : newArray}
	}
}

//overrides the == and != operators for two matrices
impl Eq for Matrix
{
	pub fn eq(&self, other : &Matrix) -> bool
	{
		if (self.height != other.height || self.width != other.width)
		{
			return false;
		}
		let mut i = 0;
		let mut j = 0;
		for self.height.times
		{
			j = 0;
			for self.width.times
			{
				if (self.values[i][j] != other.values[i][j])
				{
					return false;//immediatly exit if you know it isnt equal
				}
				j += 1;
			}
			i += 1;
		}
		return true;
	}

	pub fn ne(&self, other : &Matrix) -> bool
	{
		if (self.height != other.height || self.width != other.width)
		{
			return true;
		}
		let mut i = 0;
		let mut j = 0;
		for self.height.times
		{
			j = 0;
			for self.width.times
			{
				if (self.values[i][j] != other.values[i][j])
				{
					return true;
				}
				j += 1;
			}
			i += 1;
		}
		return false;
	}
}
