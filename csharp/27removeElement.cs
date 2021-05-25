using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;


public class _27removeElement
{
    public int RemoveElement(int[] nums, int val)
    {
        int k = 0, size = nums.Length;
        for (int i = 0; i < size; ++i)
        {
            if (nums[i] != val)
            {
                nums[k++] = nums[i];
            }
        }
        return k;
    }
}
