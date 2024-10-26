use rand::Rng;
struct Restaurant {
    name: &'static str,
    category: Category,
}

struct Category {
    name: &'static str,
    id: i8,
}

struct Categories {
    rice: Category,
    noodle: Category,
    bread: Category,
    family: Category,
    other: Category,
}

const CATEGORY: Categories = Categories {
    rice: Category { name: "米", id: 0 },
    noodle: Category {
        name: "ラーメン",
        id: 1,
    },
    bread: Category {
        name: "パン",
        id: 2,
    },
    family: Category {
        name: "ファミレス",
        id: 3,
    },
    other: Category {
        name: "その他",
        id: 4,
    },
};

const RESTAURANT: [Restaurant; 14] = [
    Restaurant {
        name: "七",
        category: CATEGORY.noodle,
    },
    Restaurant {
        name: "しぶやくん",
        category: CATEGORY.rice,
    },
    Restaurant {
        name: "ハノイのホイさん",
        category: CATEGORY.other,
    },
    Restaurant {
        name: "すき家",
        category: CATEGORY.rice,
    },
    Restaurant {
        name: "タートルバーガー",
        category: CATEGORY.bread,
    },
    Restaurant {
        name: "マクドナルド",
        category: CATEGORY.bread,
    },
    Restaurant {
        name: "長崎飯店",
        category: CATEGORY.other,
    },
    Restaurant {
        name: "ガスト",
        category: CATEGORY.family,
    },
    Restaurant {
        name: "穀雨",
        category: CATEGORY.noodle,
    },
    Restaurant {
        name: "海浜食堂",
        category: CATEGORY.rice,
    },
    Restaurant {
        name: "テング酒場",
        category: CATEGORY.rice,
    },
    Restaurant {
        name: "ガパオ食堂",
        category: CATEGORY.other,
    },
    Restaurant {
        name: "subway",
        category: CATEGORY.bread,
    },
    Restaurant {
        name: "桜丘カフェ",
        category: CATEGORY.other,
    },
];

fn main() {
    let mut input_string: String = String::new();
    println!("こんにちは！オフィス周辺でランチを選びます。\n 今日の気分はどんな感じですか？ ");
    println!("気分に合ったジャンルを入力してください。\n米(0) ラーメン(1) パン(2) ファミレス(3) その他(4):");

    std::io::stdin()
        .read_line(&mut input_string)
        .expect("行読み込みに失敗しました。");

    let input_num: i8 = input_string
        .trim()
        .parse()
        .expect("数値が入力されませんでした"); //TODO:エラーハンドリングを追加する
    if 0 >= input_num || input_num >= 4 {
        println!("0から4の数字を入力してください。");
    }

    let mut select_category_restaurants: Vec<Restaurant> = RESTAURANT
        .into_iter()
        .filter(|restaurant| restaurant.category.id == input_num)
        .collect();

    let random_restaurant: Restaurant = select_category_restaurants
        .remove(rand::thread_rng().gen_range(0..select_category_restaurants.len()));

    println!(
        "今日のランチは{}カテゴリの{}にしましょう！",
        random_restaurant.category.name, random_restaurant.name
    );
}
