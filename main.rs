mod Matrix;

fn main()
{
	loop
	{
		print("Pick an option (*all matrix values must be integers):\r\n1) Add two matrices\r\n2) Multiply a Matrix by a constant\r\n3) Multiply two matrices together\r\n4) Subtract two matricesi\r\n5)Test equal\r\n>");
		let input = io::stdin().read_line();
		match input
		{
			~"1" => addMatrices(),
			~"2" => multConst(),
			~"3" => multMatrices(),
			~"4" => subMatrices(),
			~"5" => testEqual(),
			_ => break
		};
	}
}

fn testEqual()
{
	let matrixOption1 = getMatrix();
	let matrix1 =
	match matrixOption1
	{
		None =>  
		{
			println("Invalid Matrix");
			return;
		},
		Some(Matrix::Matrix {_}) => matrixOption1.get()
	};
	let matrixOption2 = getMatrix();
	let matrix2 =
	match matrixOption2
	{
		None =>  
		{
			println("Invalid Matrix");
			return;
		},
		Some(Matrix::Matrix {_}) => matrixOption2.get()
	};
	match (matrix1 == matrix2)
	{
		true => println("They are equal!"),
		false => println("They are not equal!")
	}
}

fn subMatrices()
{
	let matrixOption1 = getMatrix();
	let matrix1 =
	match matrixOption1
	{
		None =>  
		{
			println("Invalid Matrix");
			return;
		},
		Some(Matrix::Matrix {_}) => matrixOption1.get()
	};
	let matrixOption2 = getMatrix();
	let matrix2 =
	match matrixOption2
	{
		None =>  
		{
			println("Invalid Matrix");
			return;
		},
		Some(Matrix::Matrix {_}) => matrixOption2.get()
	};
	if matrix1.canAdd(&matrix2)
	{
		let resultMatrix = matrix1 - matrix2;
		println("The resulting matrix is: ");
		resultMatrix.display();
	}
	else
	{
		println("Matrices are incorrect sizes!");
	}

}

fn multMatrices()
{
	let matrixOption1 = getMatrix();
	let matrix1 =
	match matrixOption1
	{
		None =>  
		{
			println("Invalid Matrix");
			return;
		},
		Some(Matrix::Matrix {_}) => matrixOption1.get()
	};
	let matrixOption2 = getMatrix();
	let matrix2 =
	match matrixOption2
	{
		None =>  
		{
			println("Invalid Matrix");
			return;
		},
		Some(Matrix::Matrix {_}) => matrixOption2.get()
	};
	if matrix1.canMult(&matrix2)
	{
		println("The resulting matrix is: ");
		(matrix1 * matrix2).display();
	}
	else
	{
		println("The matrices are the wrong size!");
	}

}

fn multConst()
{
	let matrixOption1 = getMatrix();
	let matrix1 =
	match matrixOption1
	{
		None =>  
		{
			println("Invalid Matrix");
			return;
		},
		Some(Matrix::Matrix {_}) => matrixOption1.get()
	};
	println("Enter const value (int)");
	let input = io::stdin().read_line();
	let valueOption = int::from_str(input);
	let value =
	match valueOption
	{
		None => 
		{
			println("Invalid input");
			return;
		},
		Some(int)=> valueOption.get()
	};
	println("The resulting matrix is: ");
	matrix1.mulConst(value).display();
}


fn addMatrices()
{
	let matrixOption1 = getMatrix();
	let matrix1 =
	match matrixOption1
	{
		None =>  
		{
			println("Invalid Matrix");
			return;
		},
		Some(Matrix::Matrix {_}) => matrixOption1.get()
	};
	let matrixOption2 = getMatrix();
	let matrix2 =
	match matrixOption2
	{
		None =>  
		{
			println("Invalid Matrix");
			return;
		},
		Some(Matrix::Matrix {_}) => matrixOption2.get()
	};
	if matrix1.canAdd(&matrix2)
	{
		let resultMatrix = matrix1 + matrix2;
		println("The resulting matrix is: ");
		resultMatrix.display();
	}
	else
	{
		println("Matrices are incorrect sizes!");
	}
}

fn getMatrix() -> Option<Matrix::Matrix>
{
	println("Enter height of you matrix: ");
	let input = io::stdin().read_line();
	let heightOption = uint::from_str(input);
	let height =
	match heightOption
	{
		None => 0,
		Some(uint)=> heightOption.get()
	};
	println("Enter width of your matrix: ");
	let widthInput = io::stdin().read_line();
	let widthOption = uint::from_str(widthInput);
	let width =
	match widthOption
	{
		None => 0,
		Some(uint) => widthOption.get()
	};
	if height == 0 || width == 0
	{
		return None;
	};
	let mut i = 0;
	let mut j = 0;
	let mut rec = ~[ ~[ 0, ..0], ..0];
	
	for height.times
	{
		j = 0;
		rec.push(~[ 0, ..0]);
		for width.times
		{
			println(fmt!("Enter value at position (%?, %?)", i, j));
			let value = io::stdin().read_line();
			let valueOption = int::from_str(value);
			
			rec[i].push(
			match valueOption
			{
				None => 0,
				Some(int) => valueOption.get()
			});
			j += 1;
		}
		i += 1;
	}
	Some(Matrix::Matrix::new(height, width, rec))
}
