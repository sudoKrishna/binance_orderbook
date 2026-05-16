const express = require("express");
const bcrypt = require("bcrypt");
const saltRounds = 10;
const myPlainTextPassword = 's0/\/\P4$$w0rD';
const someOtherPlaintextPassword = 'not_bacon';
import jwt from "jsonwebtoken";

const app = express();

app.use(express.json());

type Side = "BUY" | "SELL";

type Type = "LIMIT" | "MARKET";

type Orderbody = {
  userId : string,
  side : Side,
  type : Type,
  symbol : string,
  price? : Number,
  qty : number,
}

const USERS: any[] = [];
const STOCKS = [
    { id: 1, title: "AXIS BANK", symbol: "AXIS" },
    { id: 2, title: "HDFC BANK", symbol: "HDFC" },
    { id: 3, title: "TATA Steel", symbol: "TATA" },
];


const ORDERS = [];
const FILLS = [];
const BALANCES: any = {}; // { userId: { INR: {available, locked}, AXIS: {available, locked}, ... } }
const ORDERBOOK = {
    AXIS: { bids: {}, asks: {} },
    HDFC: { bids: {}, asks: {} },
    TATA: { bids: {}, asks: {} },
};

// --- Auth ---
app.post("/signup", (req: any, res: any) => {
    // const { username, password } = req.body;
    // 1. check username not taken
    // 2. hash password (bcrypt/argon2)
    // 3. push to USERS
    // 4. init BALANCES[userId] with INR: { available: 0, locked: 0 }
    const { username, password } = req.body;

    const existsUsername = USERS.find((user) => user.username === username);

    if (existsUsername) {
        return res.status(400).json({
            message: "Username alredy exits"
        })
    }

    bcrypt.genSalt(saltRounds, function (err: any, salt: any) {
        bcrypt.hash(password, salt, function (err: any, hash: any) {
            const newUser = {
                id: USERS.length + 1,
                username,
                password: hash,
            };

            USERS.push(newUser);

            BALANCES[newUser.id] = {
                INR: { available: 0, locked: 0 }
            };

            return res.status(201).json({
                message: "User created successfully",
                userId: newUser.id
            })
        })
    })

});

// why post because get user does not use body properly in express so post is good 

app.post("/login", async (req: any, res: any) => {
    // 1. find user by username
    // 2. compare hashed password
    // 3. return JWT / session token

    const { username, password } = req.body;

    const user = USERS.find((u) => u.username === username);

    if (!user) {
        return res.status(400).json({
            message: "fuck you user"
        });
    }

    bcrypt.compare(password, user.password, function (err: any, result: any) {

        if (!result) {
            return res.status(400).json({
                message: "Invalid password"
            });
        }

        const token = jwt.sign(
            { userId: user.id },
            "secretKey",
            { expiresIn: "1h" }
        );

        return res.status(200).json({
            message: "login hogaya",
            token,
            userId: user.id
        });

    });


});

// --- Orders ---
app.post("/order", (req: any, res: any) => {
    // body: { userId, side: "BUY"|"SELL", type: "LIMIT"|"MARKET", symbol, price?, qty }
    // 1. validate input + stock exists
    // 2. check + lock balance (INR for BUY, stock for SELL)
    // 3. run matching engine against opposite side of ORDERBOOK
    // 4. write fills to FILLS, update filledQty + status on ORDERS
    // 5. if leftover qty and LIMIT, rest on book; if MARKET, cancel remainder
    // 6. settle balances on each fill (move locked -> other asset's available)
   const body: Partial<Orderbody> = req.body;

   if(!body.userId || typeof body.userId !== "string") {
    return res.status(400).json({error : "Invalid userId"})
   }

   if(body.side !== "BUY" && body.side !== "SELL") {
    return res.status(400).json({error : "invalid side"})
   }

   if(body.type !== "LIMIT" && body.type !== "MARKET") {
     return res.status(400).json({error : "invalid type"})
   }
   if(!body.symbol || typeof body.symbol !== "string") {
    return res.status(400).json({error : "symbol check kar"})
   }
   if(typeof body.price !== "number") {
    return res.status(400).json({error : "define price as a number"})
   }
   if(!body.qty ||  typeof body.qty !== "string") {
    return res.status(400).json({error : "define qty as a number"})
   }





   const order : Orderbody = {
    userId : body.userId,
    side : body.side,
    type : body.type,
    symbol : body.symbol,
    price : body.price,
    qty : body.qty
   }

   return res.json({
    message : "OK" , order
   })


});

app.delete("/order/:orderId", (req: any, res: any) => {
    // 1. find order, check ownership
    // 2. remove from ORDERBOOK price level
    // 3. unlock remaining reserved balance
    // 4. mark status = CANCELLED
});

app.get("/orders", (req: any, res: any) => {
    // query: ?status=OPEN  (or all)
    // return current user's orders
});

// --- Market data ---
app.get("/orderbook/:symbol", (req: any, res: any) => {
    // return aggregated depth — totalQty per price level for bids and asks
    // (don't expose individual userIds to other users)
});

app.get("/fills/:symbol", (req: any, res: any) => {
    // recent trades for this stock — the "tape"
});

app.get("/stocks", (req: any, res: any) => {
    res.json(STOCKS);
});

// --- User data ---
app.get("/balance", (req: any, res: any) => {
    // return BALANCES[userId] for the authed user
});

app.listen(3000, () => console.log("CEX running on :3000"));