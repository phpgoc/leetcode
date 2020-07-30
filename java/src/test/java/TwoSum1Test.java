import org.testng.Assert;
import org.testng.annotations.Test;
import phpgoc.leetcode.TwoSum1;

public class TwoSum1Test {
    @Test
    public void test()
    {
        int[] expect_result =  {0,2};
        int[] run_result = new TwoSum1().twoSum(new int[]{2,3,5,9},7);
        Assert.assertEquals(run_result,expect_result);
    }
}
