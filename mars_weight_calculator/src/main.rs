/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: vdragomi <vdragomi@student.42wolfsburg.de> +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2021/06/09 15:15:07 by vdragomi          #+#    #+#             */
/*   Updated: 2021/06/09 15:15:08 by vdragomi         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::io;
mod mars_weight_calculator;

fn main()
{
	println!("Enter your weight (kg): ");
	let mut input_kg = String::new();

	io::stdin().read_line(&mut input_kg).unwrap();

	let weight: f32 = input_kg.trim().parse().unwrap();
	println!("Your Earth weight is: {}", weight);

	let mars_weight = mars_weight_calculator::mars_weight_calculator(weight);
	println!("Resulting weigh on Mars is: {} (kg)", mars_weight);
}
