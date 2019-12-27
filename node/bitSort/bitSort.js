/* eslint-disable no-bitwise */
/**
 * 位图排序法
 * @description 位图排序法，给定一个不重复的整数数组，输出排序后结果
 * @param {array} inArr 输入数组
 * @param {number} maxNumber 输入数组内的最大数
 */
function bitSort(inArr, maxNumber = 1e8) {
  const buf = Buffer.alloc((maxNumber >> 1) + 1);
  inArr.forEach((e) => { buf[e >> 1] += (e & 1) ? 1 : 16; });
  const out = [];
  buf.forEach((e, i) => {
    switch (e) {
      case 1:
        out.push((i << 1) + 1);
        break;
      case 16:
        out.push(i << 1);
        break;
      case 17: {
        const tmp = i << 1;
        out.push(...[tmp, tmp + 1]);
        break;
      }
      default:
        break;
    }
  });
  return out;
}

// test
const r = bitSort([1243, 345435, 34534, 76664, 645654, 76666, 76665, 345353, 3445453]);
console.log(r);
