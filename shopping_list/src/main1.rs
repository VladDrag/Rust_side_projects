/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main1.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: vdragomi <vdragomi@student.42wolfsburg.de> +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2021/06/24 15:48:33 by vdragomi          #+#    #+#             */
/*   Updated: 2021/06/25 16:21:07 by vdragomi         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::io;

fn main()
{
	let mut items:Vec<String> = vec![];
	let mut input = String::new();
	
	while input != "done"
	{
		println!("Enter an item for your shopping list:");
		input.clear();
		io::stdin().read_line(&mut input).unwrap();
		input = input.trim().to_string();
		items.push(input.clone());
	}
	items.pop();
	println!("You need to buy:");
	for j in 0..items.len()
	{
		println!("\n{}", items[j]);
		println!("_____________");
	}
}