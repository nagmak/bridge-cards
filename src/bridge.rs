use std::io;

// Pseudocode for Bridge game
// No Input

/*
	with_permutation method
		- accepts vector of unsigned integers (usize)
			- vector is a collection of order of the deck
		- returns Game object

	format_game function
		- accepts Game object
		- formats current state of the board as string
	
	main function
		- randomized shuffle of integers 1-52 (optional)
		- passes them to with_permutation 
		- then formats and prints the result
*/


type Board = Vec<Vec<String>>;
type Test = i32;

pub enum Player {
	North,
	South,
	East,
	West,
}

pub enum Suit {
	Clubs,
	Diamonds,
	Hearts,
	Spades,
}

pub enum Rank {
	Two,
	Three,
	Four,
	Five,
	Six,
	Seven,
	Eight,
	Nine,
	Ten,
	Jack,
	Queen,
	King,
	Ace
}

pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

pub struct Game {
	board: Test
	// current_turn: Turn,
}

impl Game {
	pub fn new() -> Game {
		println!("Hello");
		Game {
			board: 5
		}
	}

	pub fn play_game(&self){
		self.with_permutation([ 38, 48, 11, 6, 20, 17, 46, 8, 37, 43, 
			7, 52, 36, 10, 25, 49, 50, 16, 33, 5, 42, 32, 9, 29, 
			1, 51, 26, 18, 41, 15, 40, 31, 35, 45, 4, 12, 39, 30, 
			19, 21, 2, 34, 23, 3, 27, 22, 47, 14, 24, 28, 13, 44])
	}

	pub fn with_permutation(&self, deck: [usize; 52]) {
		for x in 0..deck.len() {
			
			// println!("{}", x);
		}
	}
}





/*
	Cards in unshuffled deck:
		- Order of Clubs, Diamonds, Hearts, Spades
		- Ordered suit as 2..10, J, Q, K, A
		- Permutation tells you which card goes into each position
			of shuffled deck
		- eg: [38, 48, 11, 6, ...] --> [KH, 10S, QC, 7C, ...]

	Players in traditional bridge:
		- South, West, North and East
		- 2 teams: North-South vs East-West
		- South deals the cards
			- Cards are dealt sequentially, starts with the person on dealer's left
			- So south will get cards in positions: 4,8,12,..,52
	
	Printing out cards:
		- highest to lowest in each suit
		- one space between each card
		- hands do no overlap
		- no spaces after end of suits in any hand

	Auction process:
		- Starts with the dealer (South), goes clockwise
			until 3 consecutive passes are registered
		- Last bid before 3 passes = trump suit
			= number of tricks that the declarer team is committing to winning


	Calculations:
		For each hand, count points:
			Ace: 4 points
			King: 3 points
			Queen: 2 points
			Jack: 1 point
			Void (empty suit): 3 points
			Singleton (suit w 1 card): 2 points
			Doubletons (suit w 2 cards): 1 point

	Play context:
		- Bidding: auction to decide which team earns points each round
		- Each bid: Is for a suit + # of tricks committed to winning
			- # of tricks + book (i.e. 6 tricks)
		Eg: Winning bid is 4 hearts by West, then E-W
		are committed to winning at least 10 tricks of 13.
		Hearts will be trump - whichever of E or W bids Hearts is declarer

		- Trick: 
			- When person who won previous trick/person clockwise
			to declarer for 1st round,
			leads a card, and other 3 players in clockwise order
			play a card (with the same suit as leading card).

			- Unless void suit, then they can play a trump (if they
			have it) or discard some other suit

			- Person who played highest card in the leading suit
				or highest trump, wins the trick and has lead

			- when all cards played, if the declarer team
				has taken at least the contracted # of tricks
				they get points

				if not, the other team gets points

		- Bid "No Trump" (NT):
			- Winning bid = NT, then normal play except:
				highest card of suit led is the winner

		- Every bid except 1st must be higher than prev bid
			higher suit at same num level
			or
			higher num level (max 7)
*/












































