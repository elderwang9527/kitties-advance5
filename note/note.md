## 220913

![](./img/2022-09-13-14-00-06.png)

### c1，

为测试修改了一些代码，但仍有如图错误。之后再了解为什么 mock 里这些地方不能照抄 runtime，而是要改为 AccountData\<u128>。dfdda

### c2，

c1 的做法错误，正确做法是将 type AccountData = () 换为 type AccountData = pallet_balances::AccountData\<Balance>\</Balance>。  
看错误提示，虽然提示没有说明之前要加 pallet_balances::，但是结合上下文，以及搜索查看提示所提到的 bound。就能推测出要写怎么写。dddi。
