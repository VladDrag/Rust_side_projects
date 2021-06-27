/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main2.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: vdragomi <vdragomi@student.42wolfsburg.de> +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2021/06/25 15:03:53 by vdragomi          #+#    #+#             */
/*   Updated: 2021/06/27 16:17:33 by vdragomi         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::io;

fn main()
{
	let mut input = String::new();
	let mut list:Vec<String> = Vec::new();

	loop
	{
		match &input[..]
		{
			"done" =>
			{
				break;
			}
			_ =>
			{
				println!("Please insert a shopping item:");
				input.clear();
				io::stdin().read_line(&mut input).unwrap();
				input = String::from(input.trim());
				list.push(input.clone());
			}
		}
	}
	list.pop();
	println!("You need to buy:");
	for j in 0..list.len()
	{
		println!("\n{}", list[j]);
		println!("_____________");
	}
}