import r from '../utils/request'

const Signin = {
    list: {},
    current: {
        bacon: 0,
        peppers: 0,
        cheese: 0,
        none: 0,
        clear() {
            Signin.current.bacon = 0
            Signin.current.none = 0
            Signin.current.cheese = 0
            Signin.current.peppers = 0
        }
    },
    all() {
        Signin.current.clear()
        return r({
            url: '/signins',
            method: 'GET'
        }).then(data => {
            data.signins.forEach(signin => {
                Signin.list[signin.id] = signin
                switch (signin.pizza) {
                    case "Cheese":
                        Signin.current.cheese++
                        break;
                    case "Peppers":
                        Signin.current.peppers++
                        break;
                    case "Bacon":
                        Signin.current.bacon++
                        break;
                    case "None":
                        Signin.current.none++
                        break;
                }
            })
        })
    },
    signin(id) {
        return Object.values(Signin.list).find(signin => signin.member_id == id)
    }
}

export default Signin