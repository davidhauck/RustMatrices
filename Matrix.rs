struct Matrix
{
	values : ~[~[int]],
	height : uint,
	width : uint
}

impl Matrix
{
	pub fn new(height : uint, width : uint, values : ~[~[int]]) -> Matrix
	{
		Matrix{height : height, width : width, values : values}
	}

	pub fn getHeight(&self) -> uint
	{
		return self.height;
	}

	pub fn get(&self, i: int, j : int) -> int
	{
		return self.values[i][j];
	}

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

	pub fn canAdd (&self, other : &Matrix) -> bool
	{
		(self.height == other.height && self.width == other.width)
	}

	pub fn canMult (&self, other : &Matrix) -> bool
	{
		(self.width == other.height)
	}
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
					return false;
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
