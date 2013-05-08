//main.rs
//main is a file to do operations on matrices.
//By: David Hauck
//Date: 5/1/13

mod Matrix;

//holds the core of the program, asks user for input and acts upon it
fn main()
{
	loop
	{
		print("Pick an option (*all matrix values must be integers):\r\n1) Add two matrices\r\n2) Subtract two matrices\r\n3) Multiply two matrices together\r\n4) Multiply a Matrix by a constant\r\n5) Test equal\r\n6) Display matrix from file\r\n>");
		let input = io::stdin().read_line();
		match input //Calls a differnet method depending on what the user wants to do
		{
			~"1" => addMatrices(),
			~"2" => subMatrices(),
			~"3" => multMatrices(),
			~"4" => multConst(),
			~"5" => testEqual(),
			~"6" => readMatrix(),
			_ => break
		};
	}
}

//Reads in a matrix from a file
fn readMatrix()
{
	let fileText : ~[~str]; //pointer to an array of strings
	let rResult : Result<@Reader, ~str>;
	rResult = io::file_reader(~path::Path("largemat.txt"));
	if rResult.is_ok()
	{
		let file = rResult.unwrap();
		fileText = file.read_lines();
	}
	else
	{
		return;
	}
	let mut i = 0;
	let mut j = 0;
	let heightOption = uint::from_str(fileText[0]); //convert the first line to a uint for the height
	let height =
	match heightOption
	{
		None => 
		{
			println("Invalid Matrix");
			return;
		},
		Some(x) => x
	};
	let widthOption = uint::from_str(fileText[1]); //convert the second line to a uint for the width
	let width =
	match widthOption
	{
		None => 
		{
			println("Invalid Matrix");
			return;
		},
		Some(x) => x
	};
	let mut rec = ~[ ~[ 0, ..0], ..0]; //pointer to an array of pointers of length 0  each to an array of integers (0) of length 0 
	for height.times
	{
		j = 0;
		rec.push(~[0, ..0]); //push a new pointer to an array of integers onto the array
		for width.times
		{
			let inputOption = int::from_str(fileText[2 + i * height + j]);
			//push the new number
			rec[i].push(
				match inputOption
				{
					None => 
					{
						println("Invalid Matrix");
						return;
					},
					Some(x) => x
				}
			);
			j += 1;
		}
		i += 1;
	}
	let newMatrix = Matrix::Matrix{height : height, width : width, values : rec};
	newMatrix.display();
}

//Tests if two user inputted matrices are equal to each other
fn testEqual()
{
	//get two matrices
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

	//test if they are equal
	match (matrix1 == matrix2)
	{
		true => println("They are equal!"),
		false => println("They are not equal!")
	}
}

//subtracts one matrix from anther
fn subMatrices()
{
	//get two matrices
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

	//if you can add them, you can subtract them
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

//multiplies two matrices together
fn multMatrices()
{
	//get two matrices
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
	//multiply them if you can
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

//multiplies a matrix by a constant
fn multConst()
{
	//get a matrix and a constant
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
		Some(x)=> x
	};
	println("The resulting matrix is: ");
	matrix1.mulConst(value).display();
}

//adds two matrices together
fn addMatrices()
{
	//get two matrices
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
	//add them together
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

//gets a matrix from user input
fn getMatrix() -> Option<Matrix::Matrix>
{
	//get the height and the width of the matrixc
	println("Enter height of you matrix: ");
	let input = io::stdin().read_line();
	let heightOption = uint::from_str(input);
	let height =
	match heightOption
	{
		None => 0,
		Some(x)=> x
	};
	println("Enter width of your matrix: ");
	let widthInput = io::stdin().read_line();
	let widthOption = uint::from_str(widthInput);
	let width =
	match widthOption
	{
		None => 0,
		Some(x) => x
	};
	if height == 0 || width == 0
	{
		return None;
	};
	//set counters
	let mut i = 0;
	let mut j = 0;
	let mut rec = ~[ ~[ 0, ..0], ..0];
	
	for height.times
	{
		j = 0;
		rec.push(~[ 0, ..0]);
		for width.times
		{
			//get each number at each position
			println(fmt!("Enter value at position (%?, %?)", i, j));
			let value = io::stdin().read_line();
			let valueOption = int::from_str(value);
			
			rec[i].push(
			match valueOption
			{
				None => 0,
				Some(x) => x
			});
			j += 1;
		}
		i += 1;
	}
	Some(Matrix::Matrix::new(height, width, rec))
}
