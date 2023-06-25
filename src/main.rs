use std::io;

#[derive(Debug)]
struct Rectangle{
    width : u32,
    height : u32
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
fn juxing(rectangle : &Rectangle) -> u32{
    rectangle.width * rectangle.height
}
fn main() {
let sq = Rectangle::square(30);
    println!("这个矩形的变长是:{:?}",sq);
    let rect1 = Rectangle {
        width: 302,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 40,
        height: 7,
    };
    let rect3 = Rectangle {
        width: 300,
        height: 500,
    };
    if rect1.can_hold(&rect2){
        println!("第一个比第二个大")
    }else if rect1.can_hold(&rect3) {
        println!("第一个比第三个大")
    }else if rect2.can_hold(&rect3) {
        println!("第二个比第三个大")
    }
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect2);
    // 使用了 dbg!可以做到一边输出一边完成里面的操作


    let rect1 = Rectangle {
        width: 30 * scale,
        height: 50,
    };
    println!("rect1 is {:#?}", rect1);
    // 普通的变量可以直接使用{} 如果要输出的是一个类型的话需要使用{:?},如果要输出另一种格式的类型用 {:#?} 
                            //  这两种方式返回的内容是一样的格式不一样


    loop {
        println!("请输入矩形的长");
        let mut chang = String::new();
        io::stdin().read_line(&mut chang).expect("Failed to read line");
        let chang: u32 = match chang.trim().parse(){
        //这里使用match是因为变成了枚举类型，如果不是枚举类型的话就不需要家match
        Ok(num) => num,
            Err(_) => {
                println!("请输入数字");
                continue;
            },
        };
        println!("请输入矩形的宽");
        let mut kuan = String::new();
        io::stdin().read_line(&mut kuan).expect("Failed to read line");
        let kuan: u32 = match kuan.trim().parse(){
            //这里使用match是因为变成了枚举类型，如果不是枚举类型的话就不需要家match
            Ok(num) => num,
            Err(_) => {
                println!("请输入数字");
                continue;
            }
        };
        let rectangle = Rectangle{
            width : chang,
            height : kuan,
        };
        println!("矩形的面积是:{}",juxing(&rectangle));
        break;
    }    
}